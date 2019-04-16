## letter_boxed_solver

A simple program in [Rust](https://www.rust-lang.org) to produce two-word solutions to the NY Times [Letter-Boxed](http://nytimes.com/puzzles/letter-boxed) puzzles, given a puzzle and a word dictionary.

The puzzle is a square with three unique (ASCII) letters per side. The player's job is to draw lines from letter to letter to makes words, eventually completing the puzzle by using all twelve letters in the puzzle. There are a couple of constraints:

1. You can start on any letter. But after the first word, each word you make must start with the last letter of the previous word.
2. Each line you draw must be to a letter on a different side.

Here's an example puzzle:

```
  --R----K----M--
  |             |
  N             U
  |             |
  A             I
  |             |
  Y             C
  |             |
  --P----H----G--
```

In this puzzle, you could not make the word `PICK`, because `I` and `C` fall on the same side. If you made the word `PINKY`, the next word you made would have to start with `Y`.

### usage

To solve the above puzzle, you could type this from the repository root:

```
cargo run --release ./words.txt RKM,UIC,PHG,NAY
```
You'd get the following output:

```
solve puzzle RKM,UIC,PHG,NAY using word list ./words.txt (567 valid words)
HACKING GRUMPY
```

You can also just build the program with `cargo build --release`, copy `target/release/letter_boxed_solver` to wherever you put your executables, then run it like:

```
letter_boxed_solver /path/to/words.txt RKM,UIC,PHG,NAY
```

### notes

* This is the first program I've written in Rust. It's probably not very good code.
* The program uses a kind of interesting "word mask" technique, where each letter present in a word is encoded as a binary digit. This allows really fast comparisons between words to see if they contain the same letters.
