# boggle-ocr

This is a program that finds as many words as possible in a 5x5 boggle puzzle.

```
$ ./get-dictionary
$ cargo run testdata/IMG_4220.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/boggle-ocr testdata/IMG_4220.txt`
found 448 words, 959 points

WORAS
STUEG
DDORQ
TETED
DASIO

   1 aero
   1 age
   1 ager
   1 ages
   1 ags
   1 are
   1 ares
   1 art
   1 arts
   2 aside
   2 aster
   1 ate
...
```

Currently, only 5x5 text files work. Any "Q" is treated as "Qu". Eventually, I want this to be able to read the letters from a picture of the board.

## Web server

Start it like this:

    make OWL2.json
    cargo run server

Get a definition like this:

    curl 'http://127.0.0.1:8000/define?word=boggle'

Count the words in a puzzle like this:

    curl 'http://127.0.0.1:8000/boggle?lines=abcde,fghij,klmno,pqrst,uvwxy'
