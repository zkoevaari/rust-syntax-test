#!/bin/bash
set -e

# Produce a list of words only present in the dictionary..
# Lines in the dictionary starting with '#', and empty lines are omitted.

DICT=keywords.txt
SRC=src/main.rs

comm -23 <(grep -Ev "^#|^$" $DICT | sort) <(tr -cs '[:alnum:]_' '\n' < $SRC | sort | uniq)
