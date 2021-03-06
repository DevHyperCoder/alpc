# ALPC

Aviation Language Phonetic Converter is a simple Rust CLI for converting letters to the corresponding phonetic alphabets in aviation language.

## Installation
> You need to have Rust toolkit installed
- Clone the repo: `git clone https://github.com/DevHyperCoder/alpc/`
- Run the program: `cargo run`

It will be made available in different Linux package repos soon.

## Usage

ALPC reads from `stdin` so, you can pipe text into APLC

`echo "hello 123" | alpc`

Also, running `alpc` will provide a input prompt.

## Features
- Convert A..Z to ALPHA..ZULU
- Convert 0..9 to ZERO..NINER

## TODO
- Only convert uppercase letters to phonetic reper. (Command line flag to override)
- Cleaner output. (Command line flag to add verbosity)
- Man page and/or `--help` argument.
