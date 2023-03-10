# Step One - Rendering

## Introduction

Open the `1_start` project in your favourite IDE. This project has a few files, but the only one you need to care about is `src/lib.rs`. This contains a simple framework for a Tic-Tac-Toe app but the logic is not implemented. The `Cell` enum represents a single cell on the Tic-Tac-Toe grid- `Nort` being the circle shape, and `Cross` being the X shape. `None` represents an empty cell. The `MoveResult` enum represents the outcome of a move which may be that the game is still ongoing, or that the previous move was illegal, or that the game has ended in a win or draw.

The `Game` struct contains the state of the game- whose turn it is and what the state of the board is. The `state` attribute represents the board in a single-dimensional array such that `0`,`1` and `2` represent the three cells of the first row, `3,`,`4` and `5` represent the three cells of the second row and finally `6`, `7` and `8` represent the last row.

There are two unimplemented methods:

* `render` should return a string with an ASCII display of the board. Each line should be three characters each of which can be a space, an `O` (Nort) or an `X` (Cross).
* `make_move` should place a piece in the given position on the board. If it is the first player's turn it should place a Nort, otherwise it should place a Cross.

The only thing that is implemented is that a new game can be created with `Game::new()`. You have one test which is at the bottom of the file (as is convention in Rust). This test is not passing yet.

## Goals

* Pass the first test
* Un-ignore the second test and make it pass
* Using the first two tests as examples, implement the remaining test cases (below) and make them pass.

## Test Cases

| Input                                 | Output                                            |
|---------------------------------------|---------------------------------------------------|
| N/A                                   | <pre>   \n</pre><pre>   \n   </pre><pre>   </pre> |
| `make_move(0,0)`                      | <pre>O  \n</pre><pre>   \n   </pre><pre>   </pre> |
| `make_move(0, 1)`                     | <pre>   \n</pre><pre>O  \n   </pre><pre>   </pre> |
| `make_move(0,0)`<br/>`make_move(1,0)` | <pre>OX \n</pre><pre>   \n   </pre><pre>   </pre> |

## Run tests

Use `cargo test` to run the tests. One test is provided to start with in `1_start`, and the later steps each have more tests.

## Run application

Use `cargo run` to run your game. To begin with it will fail because the `render` and `make_move` functions are not implemented. But once you have implemented rendering and it can understand at least some moves, you'll be able to play the game.

## Hints

* You do not need to implement `make_move` to pass the first test
* In the `render` method use `result.push` to add new characters to the end of the
* Remember to include newlines (`\n`) in the render output after each of the first two lines, to format it.
* You can iterate over `state` to get both the value of each cell and it's index like so: `for (i, cell) in self.state.iter().enumerate() {...}`
* The `unwrap` method of `MoveResult` is a convenience method that prevents you having to check the result of `make_move` every time you use it in a test, when you know it should result in a correct move.
* To address the correct cell in the one-dimensional array given a two-dimensional adddress `x,y` the correct formula is: `x + (y * 3)`
* To pass the third test you need to use `self.is_first_player_turn`.
  * It's value should invert each turn
