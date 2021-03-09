# ALPC

Aviation Language Phonetic Converter is a simple Rust CLI for converting letters to the corresponding phonetic alphabets in aviation language.

## Installation
> You need to have Rust toolkit installed
- Clone the repo: `git clone https://github.com/DevHyperCoder/alpc/`
- Run the program: `cargo run`

**Arch Linux**
`alpc` is available in the AUR  `alpc-git`

## Usage

ALPC reads from `stdin` so, you can pipe text into APLC

`echo "hello 123" | alpc`

Also, running `alpc` will provide a input prompt.

## Features
- Convert A..Z to ALPHA..ZULU
- Convert 0..9 to ZERO..NINER
- Convert only Uppercase letters to phonetics. (`-l` to convert lowercase as well)
- Clean output (Command line flag to add verbosity)

## TODO
- Man page
