# wasel-rs

An implementation in Rust of the ["Wasel algorithm"](https://en.wikipedia.org/wiki/Weasel_program)
proposed by Richard Dawkins in _The Blind Watchmaker_ to ilustrate how evolutive processes
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
./weasel-rs -a 'ABCĈDEFGĜHĤIJĴKLMNOPRSŜTUŬVZabcĉdefgĝhĥijĵklmnoprsŝtuŭvz, 👩‍👩‍👧‍👦🦦' 'Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutro 🦦'
0: Ŭĝd,ZcOUGrĈ ŭ KzBcŭvbCrlvlĝaĵ,JnpGRĴŭ👩‍👩‍👧‍👦dnvaM (2/43)
1: Ŭĝd,zaOUGrĈ ŭ KzBcŭvbCrlvlĝ👩‍👩‍👧‍👦ĵ,JnpGRĴŭ👩‍👩‍👧‍👦dnvaM (3/43)
2: Ŭĝd,zaOUGrĈ ŭ KzBcŭŬbFrlvlĝ👩‍👩‍👧‍👦ĵ,JnpGRĴŭ👩‍👩‍👧‍👦dnv M (4/43)
3: Ŭid,zaOUGro ŭ KzBŬŭŬbFrlvlĝ👩‍👩‍👧‍👦ĵ,JnpGRĴŭ👩‍👩‍👧‍👦dnv M (6/43)
...
173: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutro L (42/43)
174: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutro L (42/43)
175: Mia familio 👩‍👩‍👧‍👦 kredas, ke ĝi similas lutro 🦦 (43/43)
```

Note the correct handling of this multicode (U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F466)
family emoji.