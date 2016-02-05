Joinlines
=========

Joins two blocks of text, such that row 1 of the first block will be
appended to row 1 of the second block and so on.

Takes an argument `<lines_per_block>` which is the number of lines in a block.
Both blocks must be of equal size.

Input is read from stdin, output to stdout.

Install
=======

    cargo install joinlines

Example
=======

    # cat f.txt
    1
    2
    3
    a
    b
    c

    # joinlines 3 ',' < f.txt
    1,a
    2,b
    3,c


Test
====

  make test
