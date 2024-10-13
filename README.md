# Implementation of Huffman Tree Algorithm in Rust

011011001101110101111110111110100000101001010111101111100010100111001001000<br>
`t`000, `r`001, `e`010, `h`0110, `m`0111, `d`1000, `s`1001, `" "`101, `u`1100, `f`1101, `a`1110, `n`1111


## Table of Contents
+ [About](#about)
+ [Tech Stack](#techstack)
+ [Getting Started](#getting_started)
+ [Usage](#usage)
+ [Testing](#testing)
+ [Author](#author)

## About <a name = "about"></a>
This is a brief implementation of the Huffman tree algorithm in Rust. The project is based on the Huffman tree theory I studied to improve my understanding of how the algorithm works. The theory and algorithm will be applied in a future project. Additionally, I aim to learn how to use Rust for better performance. This is my first project utilizing Rust.


## Tech Stack <a name = "techstack"></a>

- Written in Rust Language


## Getting Started <a name = "getting_started"></a>

These instructions will guide you through installing the project on your local machine for testing purposes. 

### Requirements

This project requires `Rust 2021 edition` with `cargo` and `rustc` versions is `1.81.0`.

### Installation (Linux or MacOS)

Clone this repository
``` sh
git clone https://github.com/kevin-wijaya/Implementation-of-Huffman-Tree-Algorithm-in-Rust.git
```

Change the directory to the cloned repository
``` sh
cd Implementation-of-Huffman-Tree-Algorithm-in-Rust/
```

Run the project
``` sh
cargo run
```

## Usage <a name = "usage"></a>

To use this terminal application:

1. Input a string (e.g., `"aaabbaaccdaaddd"`) and press `[Enter]`.
2. To quit the application, type `"q"` and press `[Enter]`.

## Testing <a name = "testing"></a>

This is an example of the program running:
```sh
$ cargo run # run the application

Enter the string you want to encode ([q] to quit): 
Ba ba ba ba ba ba ba Banana Now we are right back where we have began # input string

===========================================================================

Encoded text:
111011000110000011000001100000110000011000001100000111101100110100011010000110111010111110100110101111010011011111101110111100001011011100110001011000011001011001101101011100111111011111101101011110111100001110101111011001111101100011010

Codewords:
[("a", "00"), (" ", "01"), ("b", "100"), ("w", "1010"), ("g", "10110"), ("N", "101110"), ("o", "101111"), ("i", "110000"), ("t", "110001"), ("c", "110010"), ("k", "110011"), ("n", "11010"), ("r", "11011"), ("h", "11100"), ("v", "111010"), ("B", "111011"), ("e", "1111")]

===========================================================================

Enter the string you want to encode ([q] to quit): 
q # quit the application

See you later! 
```

## Author <a name = "author"></a>
- **Kevin Wijaya** 