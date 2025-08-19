# ft_lex

## What is this?

This is an École 42 project about recreating the `lex` utility, keeping it POSIX compliant. This version is being built in Rust, for no specific reason, I just thought it would be a good project for learning Rust.

## Project Design

The design for this project is heavily inspired on DOOM's engine, each step of the lexer should be in a self-contained Rust module that expose only public I/O interfaces, the logic of the module should be private to the module.


## Reference

- [Lex man page](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/lex.html)
- [DOOM source code](https://github.dev/id-Software/DOOM)
- ...
