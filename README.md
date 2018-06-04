Joinlines
=========

Joins two blocks of text, such that row 1 of the first block will be appended
to row 1 of the second block and so on.

Input is read from stdin until EOF, output to stdout. The the first half of the
input lines will become the first block.

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

    # joinlines ',' < f.txt
    1,a
    2,b
    3,c


Test
====

  make test

TODO
====
* Support ignoring lines starting with a prefix
