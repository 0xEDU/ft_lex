# ft_lex

> If I have seen further it is by standing on the shoulders of Giants.

## What is this?

This is an École 42 project about recreating the `lex` utility, keeping it POSIX compliant. This version is being built in Rust, for no specific reason, I just thought it would be a good project for learning Rust.

## Project Design

- The design for this project is heavily inspired on DOOM's engine, each step of the lexer should be in a self-contained Rust module that expose only public I/O interfaces, the logic of the module should be private to the module.
- Error handling should be done by bubbling up a value from LexError enum

## Modules

1. Frontend module.
    - Description: Should be responsible for opening input file(s), parsing it (POSIX-compliant) and generating a data structure containing relevant information about the file
    - Input: file_name (String or Vec<String>) and options (TBD)
    - Output: TBD
2. ...

## Reference

- [Lex man page](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/lex.html)
- [DOOM source code](https://github.dev/id-Software/DOOM)
- [Rust documentation](https://doc.rust-lang.org/)
- ...
