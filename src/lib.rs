use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate
}

#[derive(Clone, Copy, Debug,PartialEq, Eq, Hash)]
pub enum PieceRole {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Color {
    White,
    Black
}

#[derive(Clone)]
pub struct Game {
    /* state, chessboard, turn, potential en passant square, the halfmove clock, and fullmove clock 
    chessboard is represented as a 2d vector of structs Piece. ep_square is a vector of length 2, which represent the coordinates of the en passant square.
    The coordinates are the row and column indexes of the square, i.e the bottom right square is [7,7].
    */
    state: GameState,
    pub chessboard: Vec<Vec<Option<Piece>>>,
    pub turn: Color,
    ep_square: Option<Vec<i8>>,
    halfmove: u64,
    fullmove: u64,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        // Start with empty board
        let mut chessboard: Vec<Vec<Option<Piece>>> = vec![vec![None; 8]; 8];
        let back_row: Vec<PieceRole> = vec![PieceRole::Rook, PieceRole::Knight, PieceRole::Bishop, PieceRole::Queen, PieceRole::King, PieceRole::Bishop, PieceRole::Knight, PieceRole::Rook];
        // Add pieces
        for i in 0..=7 {
            chessboard[0][i] = Some(Piece::new(back_row[i], Color::Black, false));
            chessboard[1][i] = Some(Piece::new(PieceRole::Pawn, Color::Black, false));
            chessboard[6][i] = Some(Piece::new(PieceRole::Pawn, Color::White, false));
            chessboard[7][i] = Some(Piece::new(back_row[i], Color::White, false));
        }
        Game {
            state: GameState::InProgress,
            chessboard: chessboard,
            turn: Color::White,
            ep_square: None,
            halfmove: 0,
            fullmove: 1
        }

    }

    /// Mutates the current board to match the given FEN (Forsyth–Edwards Notation) string.
    pub fn load_fen(&mut self, fen_string: String) -> Option<GameState> {
        // split fen string into chapters separated by spaces
        let mut placement_data: String = String::new();
        let mut active_color: String = String::new();
        let mut castling_availability: String = String::new();
        let mut en_passant: String = String::new();
        let mut halfmove_clock: String = String::new();
        let mut fullmove_clock: String = String::new();
        for (index, chapter) in fen_string.split(" ").enumerate() {
            match index {
                0 => placement_data = chapter.to_string(),
                1 => active_color = chapter.to_string(),
                2 => castling_availability = chapter.to_string(),
                3 => en_passant = chapter.to_string(),
                4 => halfmove_clock = chapter.to_string(),
                5 => fullmove_clock = chapter.to_string(),
                _ => return None
            }
        }

        // placement data
        for (row_index, row) in placement_data.split("/").enumerate() {
            let mut column_index: usize = 0;
            for char in row.chars() {
                if char.is_digit(10) {
                    for _ in 0..char.to_digit(10).unwrap() {
                        self.chessboard[row_index][column_index] = None;
                        column_index += 1;
                    }
                }
                else {
                    self.chessboard[row_index][column_index] = Some(Piece::new(
                        match char {
                            'p'|'P' => PieceRole::Pawn,
                            'r'|'R' => PieceRole::Rook,
                            'n'|'N' => PieceRole::Knight,
                            'b'|'B' => PieceRole::Bishop,
                            'q'|'Q' => PieceRole::Queen,
                            'k'|'K' => PieceRole::King,
                            _ => return None
                        },
                        if char.is_uppercase() {Color::White} else {Color::Black},
                        true
                    ));
                    let piece_clone = self.chessboard[row_index][column_index].clone().unwrap();
                        if piece_clone.role == PieceRole::Pawn {
                        match piece_clone.color {
                            Color::White => {
                                if row_index == 6 {self.chessboard[row_index][column_index].as_mut().unwrap().has_moved = false}
                            }
                            Color::Black => {
                                if row_index == 1 {self.chessboard[row_index][column_index].as_mut().unwrap().has_moved = false}
                            }
                        }
                    }
                    column_index += 1;
                }
            }
        }

        // active color
        self.turn = match active_color.as_str() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None
        };

        // castling availability
        for char in castling_availability.chars() {
            match char {
                'K' => {self.chessboard[7][4].as_mut().unwrap().has_moved = false; self.chessboard[7][7].as_mut().unwrap().has_moved = false}
                'Q' => {self.chessboard[7][4].as_mut().unwrap().has_moved = false; self.chessboard[7][0].as_mut().unwrap().has_moved = false}
                'k' => {self.chessboard[0][4].as_mut().unwrap().has_moved = false; self.chessboard[0][7].as_mut().unwrap().has_moved = false} 
                'q' => {self.chessboard[0][4].as_mut().unwrap().has_moved = false; self.chessboard[0][0].as_mut().unwrap().has_moved = false}
                '-' => (),
                _ => return None
            }
        }

        // en passant
        if en_passant != "-" {
            self.ep_square = Some(vec![56-en_passant.chars().nth(1).unwrap() as i8, en_passant.chars().nth(0).unwrap() as i8 - 97]);
        }
        else {
            self.ep_square = None;
        }

        // halfmove clock
        self.halfmove = halfmove_clock.parse::<u64>().unwrap();

        // fullmove clock
        self.fullmove = fullmove_clock.parse::<u64>().unwrap();

        return None;
    }

    // make_move calls make_move_internal so we can have an option parameter
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        return self.make_move_internal(_from, _to, false);
    }

    /// If the current game state is "InProgress" or "Check" and the move is legal, mutate the 
    /// chessboard to match the new position and return the new game state. 
    fn make_move_internal(&mut self, _from: &str, _to: &str, skip_move_check: bool) -> Option<GameState> {
        // Check that state is allowed
        if self.state == GameState::Checkmate || self.state == GameState::Stalemate {return None;}
        // Check if piece is on square, if not return None
        if self.chessboard[56-_from.chars().nth(1).unwrap() as usize][_from.chars().nth(0).unwrap() as usize - 97].is_none() {return None;}
        // Convert algebraic notation to vectors from_pos and to_pos
        let from_pos = vec![56-_from.chars().nth(1).unwrap() as i8, _from.chars().nth(0).unwrap() as i8 - 97];
        let to_pos = vec![56-_to.chars().nth(1).unwrap() as i8, _to.chars().nth(0).unwrap() as i8 - 97];
        // Clone piece, check if it's the right color, and if the move is legal
        
        let piece = self.chessboard[from_pos[0] as usize][from_pos[1] as usize].clone().unwrap();
        if !skip_move_check && piece.color != self.turn {return None;}
        if !skip_move_check && !piece.available_moves(self, from_pos.clone(), false, false).unwrap().contains(&to_pos) {return None;}
        
        // check for promotion
        if self.chessboard[from_pos[0] as usize][from_pos[1] as usize].as_ref().unwrap().role == PieceRole::Pawn && (to_pos[0] == 0 || to_pos[0] == 7) {
            if _to.len() < 3 {return None;}
            let new_role = match _to.chars().nth(2).unwrap() {
                'q'|'Q' => PieceRole::Queen,
                'r'|'R' => PieceRole::Rook,
                'n'|'N' => PieceRole::Knight,
                'b'|'B' => PieceRole::Bishop,
                _ => return None
            };
            self.chessboard[to_pos[0] as usize][to_pos[1] as usize] = Some(Piece::new(new_role, piece.color, true));
            self.chessboard[from_pos[0] as usize][from_pos[1] as usize] = None;
            self.ep_square = None;
            self.halfmove = 0;
        }
        else {
            // check if to reset the halfmove clock
            self.halfmove += 1;
            if piece.clone().role == PieceRole::Pawn || self.chessboard[to_pos[0] as usize][to_pos[1] as usize].is_some() {
                self.halfmove = 0;
            }
            // move piece
            self.chessboard[to_pos[0] as usize][to_pos[1] as usize] = Some(Piece::new(piece.role, piece.color, true));
            self.chessboard[from_pos[0] as usize][from_pos[1] as usize] = None;

            
            // if en passant
            if self.ep_square != None && self.ep_square == Some(to_pos.clone()) && piece.role == PieceRole::Pawn {
                self.chessboard[((self.ep_square.clone().unwrap()[0]+7)/3) as usize][self.ep_square.clone().unwrap()[1] as usize] = None;
            }
            // if pawn double stepped
            if piece.clone().role == PieceRole::Pawn && (to_pos[0] - from_pos[0]).abs() == 2 {
                // set en passant square to the square behind the pawn
                self.ep_square = Some(vec![from_pos[0] + (to_pos[0] - from_pos[0])/2, from_pos[1]]);
            }
            else {
                // reset en passant square
                self.ep_square = None;
            }

            // if castling
            if piece.clone().role == PieceRole::King && (to_pos[1] - from_pos[1]).abs() == 2 {
                self.chessboard[to_pos[0] as usize][(to_pos[1] - (to_pos[1] - from_pos[1]).signum()) as usize] = Some(Piece::new(PieceRole::Rook, piece.color, true));
                self.chessboard[to_pos[0] as usize][((7*to_pos[1]-14)/4) as usize] = None;

            }

        }

        if skip_move_check {return None;}
        
        // change state depending on check
        if Game::in_check(&self, if self.turn == Color::White {Color::Black} else {Color::White}) {
            self.state = GameState::Check;
        } else {    
            self.state = GameState::InProgress;
        }

        // look for checkmate and stalemate
        for (row_index, row) in self.chessboard.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                match piece {
                    Some(piece) => {
                        if piece.color != self.turn && piece.available_moves(self, vec![row_index as i8, column_index as i8], false, false).unwrap().len() > 0 {
                            // change fullmove clock after every black turn
                            if self.turn == Color::Black {self.fullmove += 1;}
                            self.turn = if self.turn == Color::White {Color::Black} else {Color::White};
                            return Some(self.state);
                        }
                    }
                    None => ()
                }
            }
        }
        // no moves are available, meaning that the game is either checkmate or stalemate
        if Game::in_check(&self, if self.turn == Color::White {Color::Black} else {Color::White}) {
            self.state = GameState::Checkmate;
        } else {
            self.state = GameState::Stalemate;
        }
        if self.turn == Color::Black {self.fullmove += 1};
        return Some(self.state);
    }

    /// Get the halfmove clock.
    pub fn get_halfmove(&self) -> u64 {
        return self.halfmove;
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        return self.state;
    }

    /// Get the current turn.
    pub fn get_turn(&self) -> &str {
        match self.turn {
            Color::White => "White",
            Color::Black => "Black"
        }
    }

    /// Get the FEN (Forsyth–Edwards Notation) string of the current board position.
    pub fn get_fen(&self) -> String {
        // split fen string into chapters
        let mut placement_data: String = String::new();
        let mut active_color: String = String::new();
        let mut castling_availability: String = String::new();
        let mut en_passant: String = String::new();
        let mut halfmove_clock: String = String::new();
        let mut fullmove_clock: String = String::new();

        // placement data
        for row in self.chessboard.iter() {
            let mut row_string: String = String::new();
            let mut empty_squares: i8 = 0;
            for piece in row.iter() {
                if piece.is_some() {
                    let piece = piece.clone().unwrap();
                    if empty_squares > 0 {row_string.push_str(&empty_squares.to_string()); empty_squares = 0;}
                    row_string.push_str(
                match piece.role {
                        PieceRole::Pawn => if piece.color == Color::White {"P"} else {"p"},
                        PieceRole::Rook => if piece.color == Color::White {"R"} else {"r"},
                        PieceRole::Knight => if piece.color == Color::White {"N"} else {"n"},
                        PieceRole::Bishop => if piece.color == Color::White {"B"} else {"b"},
                        PieceRole::Queen => if piece.color == Color::White {"Q"} else {"q"},
                        PieceRole::King => if piece.color == Color::White {"K"} else {"k"}
                        }
                    );
                }
                else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {row_string.push_str(&empty_squares.to_string())}
            row_string.push_str("/");
            placement_data.push_str(&row_string);
        }
        placement_data.pop();

        // active color
        active_color.push_str(match self.turn {
            Color::White => "w",
            Color::Black => "b"
        });

        // castling availability
        if self.chessboard[7][4].is_some() && self.chessboard[7][4].as_ref().unwrap().role == PieceRole::King && !self.chessboard[7][4].as_ref().unwrap().has_moved {  
            if self.chessboard[7][7].is_some() && self.chessboard[7][7].as_ref().unwrap().role == PieceRole::Rook && !self.chessboard[7][7].as_ref().unwrap().has_moved {
                castling_availability.push_str("K");
            }
            if self.chessboard[7][0].is_some() && self.chessboard[7][0].as_ref().unwrap().role == PieceRole::Rook && !self.chessboard[7][0].as_ref().unwrap().has_moved {
                castling_availability.push_str("Q");
            }
        }
        if self.chessboard[0][4].is_some() && self.chessboard[0][4].as_ref().unwrap().role == PieceRole::King && !self.chessboard[0][4].as_ref().unwrap().has_moved {  
            if self.chessboard[0][7].is_some() && self.chessboard[0][7].as_ref().unwrap().role == PieceRole::Rook && !self.chessboard[0][7].as_ref().unwrap().has_moved {
                castling_availability.push_str("k");
            }
            if self.chessboard[0][0].is_some() && self.chessboard[0][0].as_ref().unwrap().role == PieceRole::Rook && !self.chessboard[0][0].as_ref().unwrap().has_moved {
                castling_availability.push_str("q");
            }
        }
        if castling_availability.len() == 0 {castling_availability.push_str("-");}

        // en passant
        if self.ep_square != None {
            en_passant.push_str(&format!("{}{}", (97+self.ep_square.clone().unwrap()[1]) as u8 as char, (56-self.ep_square.clone().unwrap()[0]) as u8 as char));
        }
        else {
            en_passant.push_str("-");
        }

        // halfmove clock
        halfmove_clock.push_str(&self.halfmove.to_string());

        // fullmove clock
        fullmove_clock.push_str(&self.fullmove.to_string());

        // add all chapters together separated by spaces

        return format!("{} {} {} {} {} {}", placement_data, active_color, castling_availability, en_passant, halfmove_clock, fullmove_clock);

    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece.
    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {
        // Check if state is allowed
        if self.state == GameState::Checkmate || self.state == GameState::Stalemate {return None;}
        // Convert from algebraic notation to vector
        let pos = vec![56-_position.chars().nth(1).unwrap() as i8, _position.chars().nth(0).unwrap() as i8 - 97]; 
        // Check that piece is on square
        if self.chessboard[pos[0] as usize][pos[1] as usize].is_none() {return None;}
        // Clone piece
        let piece = self.chessboard[pos[0] as usize][pos[1] as usize].as_ref().unwrap();

        // convert all possible moves to algebraic notation
        let mut moves_algebraic: Vec<String> = Vec::new();
        for move_vec in piece.available_moves(&self, pos.clone(), false, false).unwrap() {
            moves_algebraic.push(format!("{}{}", (97+move_vec[1]) as u8 as char, (56-move_vec[0] as u8) as char));
        }
        return Some(moves_algebraic);
    }

    /// Returns either true or false if the given color is in check.
    fn in_check(board: &Game, _turn: Color) -> bool {
        // find king position
        let mut king_pos: Vec<i8> = Vec::new();
        'find_king: for (row_index, row) in board.chessboard.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                match piece {
                    Some(piece) => {
                        if piece.color == _turn && piece.role == PieceRole::King {
                            king_pos = vec![row_index as i8, column_index as i8];
                            break 'find_king;
                        }
                    }
                    None => ()
                }
            }
        }

        // check if any enemy piece can attack the king position, if so return true
        for (row_index, row) in board.chessboard.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                match piece {
                    Some(piece) => {
                        if piece.color != _turn {
                            for pos in  piece.available_moves(&board, vec![row_index as i8, column_index as i8], true, true).unwrap() {
                                if pos == king_pos {
                                    return true;
                                }
                            }
                        }
                    }
                    None => ()
                }
            }
        }
        return false
    }


}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub role: PieceRole,
    pub color: Color,
    has_moved: bool,
}

impl Piece {
    // Every piece has an enum role, color, and a bool if it has moved or not, which is only used for castling
    fn new(role: PieceRole, color: Color, has_moved: bool) -> Piece {
        Piece {
            role,
            color,
            has_moved,
        }
    }
    fn available_moves(&self, game:&Game, pos: Vec<i8>, only_attack_moves: bool, ignore_check: bool) -> Option<Vec<Vec<i8>>> {
        //println!("huh");
        fn move_okay(move_vec: Vec<i8>) -> bool {return move_vec[0] >= 0 && move_vec[0] <= 7 && move_vec[1] >= 0 && move_vec[1] <= 7;}
        let board = &game.chessboard;
        let mut moves: Vec<Vec<i8>> = Vec::new();
        // color as i8
        match self.role {
            PieceRole::Pawn => {
                // -1 for white, 1 for black
                let white_black: i8 = if self.color == Color::White {-1} else {1};
                // check diagonally left
                if move_okay(vec![pos[0]+white_black, pos[1] - 1]) && (board[(pos[0]+white_black) as usize][(pos[1] - 1) as usize].is_some() || game.ep_square == Some(vec![pos[0]+white_black,pos[1]-1])) {moves.push(vec![pos[0]+white_black, pos[1] - 1])}
                
                // check diagonally right
                if move_okay(vec![pos[0]+white_black, pos[1] + 1]) && (board[(pos[0]+white_black) as usize][(pos[1] + 1) as usize].is_some()|| game.ep_square == Some(vec![pos[0]+white_black,pos[1]+1])) {moves.push(vec![pos[0]+white_black, pos[1] + 1])}
                
                // check one ahead
                if !only_attack_moves && move_okay(vec![pos[0]+white_black, pos[1]]) && board[(pos[0]+white_black) as usize][(pos[1]) as usize].is_none() {
                    moves.push(vec![pos[0]+white_black, pos[1]]);
                    // check two ahead
                    if !self.has_moved && move_okay(vec![pos[0]+2*white_black,pos[1]]) && board[(pos[0]+2*white_black) as usize][(pos[1]) as usize].is_none() {
                        moves.push(vec![pos[0]+2*white_black, pos[1]]);
                    }
                }
            }
            PieceRole::Rook => {
                // directions are up, right, down, left (clockwise)
                let mut dir_bools: Vec<bool> = vec![true, true, true, true];
                let dir_vectors: Vec<Vec<i8>> = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];  
                for offset in 1..=7 {
                    for dir_index in 0..=3 {
                        if dir_bools[dir_index] == false {continue}
                        // check if move is okay
                        if move_okay(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]) {
                            // add move
                            moves.push(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]);
                            // if piece is in the way, stop checking that direction
                            if board[(pos[0]+offset*dir_vectors[dir_index][0]) as usize][(pos[1]+offset*dir_vectors[dir_index][1]) as usize].is_some() {
                                dir_bools[dir_index] = false;
                            }
                        }
                    }
                }
            }
            PieceRole::Knight => {
                // check all squares clockwise
                if move_okay(vec![pos[0]-2, pos[1]+1]) {moves.push(vec![pos[0]-2, pos[1]+1])}
                if move_okay(vec![pos[0]-1, pos[1]+2]) {moves.push(vec![pos[0]-1, pos[1]+2])}
                if move_okay(vec![pos[0]+1, pos[1]+2]) {moves.push(vec![pos[0]+1, pos[1]+2])}
                if move_okay(vec![pos[0]+2, pos[1]+1]) {moves.push(vec![pos[0]+2, pos[1]+1])}
                if move_okay(vec![pos[0]+2, pos[1]-1]) {moves.push(vec![pos[0]+2, pos[1]-1])}
                if move_okay(vec![pos[0]+1, pos[1]-2]) {moves.push(vec![pos[0]+1, pos[1]-2])}
                if move_okay(vec![pos[0]-1, pos[1]-2]) {moves.push(vec![pos[0]-1, pos[1]-2])}
                if move_okay(vec![pos[0]-2, pos[1]-1]) {moves.push(vec![pos[0]-2, pos[1]-1])}
            }
            PieceRole::Bishop => {
                // directions are up-right, down-right, down-left, up-left (clockwise)
                let mut dir_bools: Vec<bool> = vec![true, true, true, true];
                let dir_vectors: Vec<Vec<i8>> = vec![vec![-1, 1], vec![1, 1], vec![1, -1], vec![-1, -1]];
                for offset in 1..=7 {
                    for dir_index in 0..=3 {
                        if dir_bools[dir_index] == false {continue}
                        // check if move is okay
                        if move_okay(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]) {
                            // add move
                            moves.push(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]);
                            // if piece is in the way, stop checking that direction
                            if board[(pos[0]+offset*dir_vectors[dir_index][0]) as usize][(pos[1]+offset*dir_vectors[dir_index][1]) as usize].is_some() {
                                dir_bools[dir_index] = false;
                            }
                        }
                    }
                }
            }
            PieceRole::Queen => {
                // directions are up, up-right, right, down-right, down, down-left, left, up-left (clockwise)
                let mut dir_bools: Vec<bool> = vec![true, true, true, true, true, true, true, true];
                let dir_vectors: Vec<Vec<i8>> = vec![vec![-1, 0], vec![-1, 1], vec![0, 1], vec![1, 1], vec![1, 0], vec![1, -1], vec![0, -1], vec![-1, -1]];
                for offset in 1..=7 {
                    for dir_index in 0..=7 {
                        if dir_bools[dir_index] == false {continue}
                        // check if move is okay
                        if move_okay(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]) {
                            // add move
                            moves.push(vec![pos[0]+offset*dir_vectors[dir_index][0], pos[1]+offset*dir_vectors[dir_index][1]]);
                            // if piece is in the way, stop checking that direction
                            if board[(pos[0]+offset*dir_vectors[dir_index][0]) as usize][(pos[1]+offset*dir_vectors[dir_index][1]) as usize].is_some() {
                                dir_bools[dir_index] = false;
                            }
                        }
                    }
                }
            }
            PieceRole::King => {
                // check all squares clockwise
                //println!("UNO {:?}",self);
                if move_okay(vec![pos[0]-1, pos[1]]) {moves.push(vec![pos[0]-1, pos[1]])}
                if move_okay(vec![pos[0]-1, pos[1]+1]) {moves.push(vec![pos[0]-1, pos[1]+1])}
                if move_okay(vec![pos[0], pos[1]+1]) {moves.push(vec![pos[0], pos[1]+1])}
                if move_okay(vec![pos[0]+1, pos[1]+1]) {moves.push(vec![pos[0]+1, pos[1]+1])}
                if move_okay(vec![pos[0]+1, pos[1]]) {moves.push(vec![pos[0]+1, pos[1]])}
                if move_okay(vec![pos[0]+1, pos[1]-1]) {moves.push(vec![pos[0]+1, pos[1]-1])}
                if move_okay(vec![pos[0], pos[1]-1]) {moves.push(vec![pos[0], pos[1]-1])}
                if move_okay(vec![pos[0]-1, pos[1]-1]) {moves.push(vec![pos[0]-1, pos[1]-1])}

                // queenside castling
                if !ignore_check && !Game::in_check(&Game{state: GameState::InProgress, chessboard: board.clone(), turn: self.color, ep_square:None, halfmove:0, fullmove:1}, self.color) && !self.has_moved && board[pos[0] as usize][(pos[1]-4) as usize].is_some()
                && board[pos[0] as usize][(pos[1]-4) as usize].as_ref().unwrap().role == PieceRole::Rook && !board[pos[0] as usize][(pos[1]-4) as usize].as_ref().unwrap().has_moved {
                    for i in 1..=3 {
                        if board[pos[0] as usize][(pos[1]-i) as usize].is_some() {break;}
                        if i != 3 {
                            let board_copy = &mut Game{state: GameState::InProgress, chessboard: board.clone(), turn: self.color, ep_square:None, halfmove:0, fullmove:1};
                            board_copy.chessboard[pos[0] as usize][pos[1] as usize] = None;
                            board_copy.chessboard[pos[0] as usize][(pos[1]-i) as usize] = Some(Piece::new(PieceRole::King, self.color, true));
                            if Game::in_check(board_copy, self.color) {break;}
                        }
                        else {moves.push(vec![pos[0], (pos[1]-2)])}
                    }
                }

                // kingside castling
                if !ignore_check && !Game::in_check(&Game{state: GameState::InProgress, chessboard: board.clone(), turn: self.color, ep_square:None, halfmove:0, fullmove:1}, self.color) && !self.has_moved && board[pos[0] as usize][(pos[1]+3) as usize].is_some()
                && board[pos[0] as usize][(pos[1]+3) as usize].as_ref().unwrap().role == PieceRole::Rook && !board[pos[0] as usize][(pos[1]+3) as usize].as_ref().unwrap().has_moved {
                    for i in 1..=2 {
                        if board[pos[0] as usize][(pos[1]+i) as usize].is_some() {break;}
                        let board_copy = &mut Game{state: GameState::InProgress, chessboard: board.clone(), turn: self.color, ep_square: None, halfmove:0, fullmove:1};
                        board_copy.chessboard[pos[0] as usize][pos[1] as usize] = None;
                        board_copy.chessboard[pos[0] as usize][(pos[1]+i) as usize] = Some(Piece::new(PieceRole::King, self.color, true));
                        if Game::in_check(board_copy, self.color) {break;}
                        if i == 2 {moves.push(vec![pos[0], (pos[1]+2)])}
                    }
                }
            }
        }
        

        // remove squares with own color (is_none() prevents error when accessing None)
        moves.retain(|x| board[x[0] as usize][x[1] as usize].is_none() || board[x[0] as usize][x[1] as usize].as_ref().unwrap().color != self.color);
        //println!("DOS {:?} {:?}",self, moves);
        // remove squares that would put king in check
        if ignore_check {return Some(moves)}
        let moves_copy = moves.clone();
        for move_vec in moves_copy {
            let mut board_copy = game.clone();
            board_copy.make_move_internal(&format!("{}{}", (97+pos[1]) as u8 as char, (56-pos[0]) as u8 as char), &format!("{}{}", (97+move_vec[1]) as u8 as char, (56-move_vec[0]) as u8 as char), true);
            println!("{:?}", board_copy);
            //board_copy[move_vec[0] as usize][move_vec[1] as usize] = Some(Piece::new(self.role, self.color, true));
            //board_copy[pos[0] as usize][pos[1] as usize] = None;
            if Game::in_check(&board_copy, self.color) {
                moves.remove(moves.iter().position(|x| *x == move_vec).unwrap());
            }
        }

        return Some(moves);
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        for row in self.chessboard.iter() {
            for piece in row.iter() {
                match piece {
                    Some(piece) => {let debug_notation: &str = 
                        match piece.role {
                            PieceRole::Pawn => "p",
                            PieceRole::Rook => "r",
                            PieceRole::Knight => "n",
                            PieceRole::Bishop => "b",
                            PieceRole::Queen => "q",
                            PieceRole::King => "k"
                        };
                        board_string += &(if piece.color == Color::White {debug_notation.to_uppercase()} else {debug_notation.to_lowercase()});
                        board_string.push_str(" ");
                    }
                    None => board_string.push_str("* ")
                }
            }
            board_string.push_str("\n");
        }
        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();
        println!("{:?}", game);
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    //check that loading a fen-string works
    #[test]
    fn check_load_fen() {
        let game1 = Game::new();
        let mut game2 = Game::new();
        game2.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        assert_eq!(format!("{:?}",game1), format!("{:?}",game2));
    }

    //check that making a fen-string works
    #[test]
    fn check_get_fen() {
        let game1 = Game::new();
        let mut game2 = Game::new();
        game2.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        assert_eq!(game1.get_fen(), game2.get_fen());
    }

    //check that making a move works
    #[test]
    fn check_make_move() {
        let mut game1 = Game::new();
        let mut game2 = Game::new();
        game2.load_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string());
        game1.make_move("e2", "e4");
        assert_eq!(format!("{:?}",game1), format!("{:?}",game2));
    }

    //check that getting halfmove works
    #[test]
    fn check_get_halfmove() {
        let mut game1 = Game::new();
        game1.make_move("b1", "a3");
        assert_eq!(game1.get_halfmove(), 1);
    }

    //check that getting turn works
    #[test]
    fn check_get_turn() {
        let mut game1 = Game::new();
        game1.make_move("b1", "a3");
        assert_eq!(game1.get_turn(), "Black");
    }

    //check that getting possible moves works
    #[test]
    fn check_get_possible_moves() {
        let game1 = Game::new();
        println!("{:?}", game1.get_possible_moves("b1"));
        assert_eq!(game1.get_possible_moves("b1"), Some(vec!["c3".to_string(),"a3".to_string()]));
    }

    //check that checking for check works
    #[test]
    fn check_check() {
        let game1 = Game::new();
        assert_eq!(Game::in_check(&game1, game1.turn), false); 
    }
}