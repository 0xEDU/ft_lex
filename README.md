# ft_lex

> If I have seen further it is by standing on the shoulders of Giants.

## What is this?

This is an École 42 project about recreating the `lex` utility, keeping it POSIX compliant. This version is being built in Rust, for no specific reason, I just thought it would be a good project for learning Rust.

## How it works?

Basically `lex` will parse a `lexer.l` source file, which is divided into 3 sections: Definitions, Rules and UserSubroutines:

1. Definitions will be copied to the final file as is (if they are as described in the specification), dunno what to do with substitution strings yet;
2. Rules will be treated somewhat similarly, code in the prelude will be copied to a yylex() function, the regex rules will go through a regex engine and the output will be written to yylex() in a sort of if rule then action;
3. UserSubroutines will be copied to the final file as is.

## Project Design

- The design for this project is heavily inspired on DOOM's engine, each step of the lexer should be in a self-contained Rust module that expose only an `invoke()` function and it's I/O types, the logic of the module should be private to the module.
- Error handling should be done by bubbling up a value from LexError enum.

## Modules

1. Input Loader.
    - Description: Should be responsible for handling options, opening and parsing (POSIX-compliant) input file(s) and generating a data structure containing relevant information about the file.
    - Input: Command line arguments.
    - Output: An Intermediate Representation of the parsed lex source.
2. ...

## Reference

- [Lex man page](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/lex.html)
- [DOOM source code](https://github.dev/id-Software/DOOM)
- [Rust documentation](https://doc.rust-lang.org/)
- ...
