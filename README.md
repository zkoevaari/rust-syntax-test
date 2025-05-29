rust-syntax-test
================

Goal of this project is to provide a Rust source file that exercises most
language features, so it can be used in the development of syntax
highlighters.

**Note:** Besides the intended purpose, the program can be built and run,
but it is silly, sub-optimal, and not very pretty.


## Status ##

The file can be considered feature-complete, and in accordance with the Rust
Reference as of language edition 2024 (version 1.85.0).

Keyword-completeness can be checked with the attached script (its output
should be empty).


## Known issues ##

- Reserved keywords are not incorporated, they are disabled in the list.
- Types `f16` and `f128` are included, in expectation.
