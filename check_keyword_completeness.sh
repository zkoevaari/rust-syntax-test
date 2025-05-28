#!/bin/bash
set -e

# Produce a list of words only present in the first input.
# Lines in the dictionary starting with '#', and empty lines are omitted.

DICT=keywords.txt
SRC=src/main.rs

#comm -23 <(tr '[:upper:]' '[:lower:]' < $DICT | grep -Ev "^#|^$" | sort) <(tr -cs '[:alpha:]' '\n' < $SRC | tr '[:upper:]' '[:lower:]' | sort | uniq)
comm -23 <(grep -Ev "^#|^$" $DICT | sort) <(tr -cs '[:alnum:]_' '\n' < $SRC | sort | uniq)
