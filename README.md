# weasel-rs

An implementation in Rust of the ["Weasel algorithm"](https://en.wikipedia.org/wiki/Weasel_program)
proposed by Richard Dawkins in _The Blind Watchmaker_, to ilustrate how evolutionary processes
differ from complete random chance.

Basic use is just to run by itself. Example (abridged):

```
./weasel-rs
0: teSyNuqZWJUbewBvkhwwptEgJizD (0/28)
1: teSyNuqZWJGbewBvkhwwptWgJihD (1/28)
2: teSyNuqZWJGbew vkhwwptWgJshD (2/28)
3: teSyNuqZWJGbew DIhwwptWgJshD (3/28)
...
134: METHINKS IT IS BIKE A WEASEL (27/28)
135: METHINKS IT IS BIKE A WEASEL (27/28)
136: METHINKS IT IS LIKE A WEASEL (28/28)
```

Run with the `-h` flag to get parameters and their defaults. (Current defaults
are mostly the ones given in the [example of the Wikipedia article](https://en.wikipedia.org/wiki/Weasel_program#Example_algorithm),
with the exception that the default alphabet also has lowecase letters).

Note that you can define your own alphabet, and an attempt was made to handle
at least some Unicode characters correctly (make sure your terminal can handle them, too).
Example (abridged):

```
./weasel-rs -a 'ABCĈDEFGĜHĤIJĴKLMNOPRSŜTUŬVZabcĉdefgĝhĥijĵklmnoprsŝtuŭvz, 👩‍👩‍👧‍👦🦦' 'Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutron 🦦'
0: MJZO🦦,zrciIm🦦ŜĜNICA eJrĉalnDduPhno🦦VzŭOUlmmL (2/44)
1: MJZOI,zrciIm🦦ŜĜNICA eJrĉĜlnDduPhno🦦VzŭGUlmmL (2/44)
2: MJZOI,mrciIm🦦ŜĜNICA eJreĜlnDduPhno🦦VzŭGUlmmL (4/44)
3: MJZOIĵmrciIm🦦ŜĜNICA eJreĜlnDduPhno🦦VzŭGjlmmL (4/44)
...
579: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lFtron 🦦 (43/44)
580: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lFtron 🦦 (43/44)
581: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutron 🦦 (44/44)
```

Note the correct handling of this multicode (U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F466)
family emoji.