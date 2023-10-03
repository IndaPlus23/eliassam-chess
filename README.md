## eliassam_chess_lib documentation

### struct Game
The entire chess game is stored in a struct called Game. You interact with this struct through the public functions and methods that are documented below.

### Public functions and methods

#### pub fn new() -> Game
Initializes and returns a new board with standard pieces. Make sure it is declared as mutable.

**Arguments**  
This method takes no arguments.

**Example**
```
let mut new_game = eliassam_chess_lib::Game::new();
```

#### pub fn get_game_state(&self) -> GameState 
Gets the current game state as enum GameState. The enums are   
InProgress, Check, Checkmate, Stalemate.

**Arguments**  
get_game_state is a method and only takes an instance of Game as an argument.

**Example**
```
if new_game.get_game_state() == eliassam_chess_lib::GameState::Checkmate {
    println!("Checkmate!");
}

```

#### pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>>

Gets all the possible moves for piece on square _position. Returns vector of Strings with squares in algebraic notation, or None if no legal moves. Castling is only seen as an available move for the king, not for the rooks.

**Arguments**  
get_possible_moves takes an instance of Game and the position of the piece in algebraic notation as a string slice. That means that _position can *only* be a combination of letters a-h, and numbers 1-8, e.g. f3.

**Example**
```
println!("The possible moves for b1 are {:?}.", new_game.get_possible_moves("b1").unwrap());

> The possible moves for b1 are ["c3", "a3"].
```

#### pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState\>
Mutates the chessboard to make a legal move. Returns the enum GameState, or None if something failed in the method (like trying to do an illegal move, or black is trying to move when it is white's turn).

**Arguments**  
make_move takes an instance of Game and two positions (both in algebraic notation): from where the piece moves, and to where it will move. Algebraic notation means that _from and _to can *only* be a combination of the letters a-h, and numbers 1-8, e.g. f3.

The one exception to this is when promoting a pawn. Then _to must be written as the target square in algebraic notation, followed by the desired promotion where  
Q => Queen  
R => Rook  
N => Knight  
B => Bishop  
i.e. d8Q (Moves pawn to d8 and promotes to queen). Both upper- and lowercase works.

**Example**
```
new_game.make_move("d2","d4")
```

#### pub fn get_turn(&self) -> &str

Gets which player's turn it is and returns either "White" or "Black" as a string slice.

**Arguments**  
get_turn is a method and only takes an instance of Game as an argument.


**Example**

```
println!("{} to move.", new_game.get_turn());

> White to move.
```

#### pub fn get_halfmove(&self) -> u64

Gets the halfmove clock as an unsigned 64-bit integer. Used for enforcing the fifty-move rule.

**Arguments**  
get_halfmove is a method and only takes an instance of Game as an argument.


**Example**

```
println!("There has been {} moves since a capture or a pawn moved.",
new_game.get_halfmove());

> There has been 2 moves since a capture or a pawn moved.
```

#### pub fn get_fen(&self) -> String

Gets the FEN (Forsyth–Edwards Notation) string of the game and returns it as String.

**Arguments**  
get_fen is a method and only takes an instance of Game as an argument.


**Example**

```
println!("Copy the FEN string to save your game: \n{}", new_game.get_fen());

> Copy the FEN string to save your game: 
> r1bqkbnr/pp1ppppp/n7/1N6/1p6/P7/2PPPPPP/R1BQKBNR b KQkq - 1 4
```

#### pub fn load_fen(&mut self, fen_string: String) -> Option<GameState\>

Mutates Game to match the FEN (Forsyth–Edwards Notation) String, and returns the current enum GameState.

**Arguments**  
load_fen takes an instance of Game and a FEN-string as String.


**Example**

```
let mut new_game = eliassam_chess_lib::Game::new();
new_game.load_fen(
"r1bqkbnr/pp1ppppp/n7/1N6/1p6/P7/2PPPPPP/R1BQKBNR b KQkq - 1 4"
.to_string());
```