# The Five-Letter Problem in 0.79 Seconds

@davidad, _August 2022_

![MIT License](https://img.shields.io/github/license/davidad/five-letters)

## Background

Someone (Daniel Bingham) submitted this puzzle to YouTuber [Matt Parker](https://www.youtube.com/user/standupmaths)'s podcast [A Problem Squared](https://aproblemsquared.libsyn.com/):

> Can you find five five-letter words with twenty-five unique letters?

You should watch Matt's [excellent video](https://www.youtube.com/watch?v=_-AfhLQfb6w) about the puzzle, but the gist is that he wrote some code that he knew full well was terribly inefficient, let it run on an old laptop for literally over a month continuously while he worked on other things, and then got his answers. Then he talked about it on [that podcast](https://aproblemsquared.libsyn.com/038-fldxt-in-wordle-and-improv-tact-hurdle), and then Benjamin Paassen built an [improved algorithm](https://gitlab.com/bpaassen/five_clique/-/tree/main/) which brought the running time down to just a little over 15 minutes.

### A Challenge!

Although a great improvement, Paassen's code is still in Python and it's still pretty straightforward. I bet there was at least another solid order of magnitude improvement possible. This is what I call a fun Friday night activity.

## Contributions

I have a newer laptop, so I also clocked the performance of Paassen's original Python code on my machine: it's about **14m16s**, so only a small improvement from hardware alone.

### Rewrite It In Rust

I took Paassen's exact algorithmic strategy and implemented it in a performant manner, in Rust. The runtime of that strategy is just a hair under 200 seconds, or **3m20s**:
![Benchmark plot showing median 199.14 seconds](./doc/img/clique_search_report.png)

### Dancing Links

But then I pulled out my copy of [Volume 4B, Fascicle 5](https://blackwells.co.uk/bookshop/product/The-Art-of-Computer-Programming-Volume-4B-Fascicle-5-Mathematical-Preliminaries-Redux-Backtracking-Dancing-Links-by-Donald-Ervin-Knuth/9780134671796) of [Donald Knuth](https://en.wikipedia.org/wiki/Donald_Knuth)'s [The Art of Computer Programming](https://en.wikipedia.org/wiki/The_Art_of_Computer_Programming) and implemented [Knuth's Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) (that's its actual name!) with his [Dancing Links](https://en.wikipedia.org/wiki/Dancing_Links) optimizations.

Using Algorithm X, the entire solution benchmarks under **0.795 s**:

![Benchmark plot showing median 790.96 ms](./doc/img/dancing_links_report.png)

## How does it work?

I encourage you to read Knuth's own explanation, but here's a brief summary. Dancing Links is a way of making backtracking deletions from lists really efficient, by using doubly-linked lists and exploiting the fact that "undeletions" can be guaranteed to happen in exactly the reverse order from deletions. Algorithm X uses one horizontal doubly-linked list (to tie together, in this case, the letters that are still available) and for each letter, one vertical doubly-linked list (to tie together all the words that would use that letter):

![Diagram of Dancing Links data structure for Algorithm X](./doc/img/dancing_links_structure.png)

_image credit: "[Solving Sudoku efficiently with Dancing Links](https://www.kth.se/social/files/58861771f276547fe1dbf8d1/HLaestanderMHarrysson_dkand14.pdf)", Harrysson and Laestander, 2014_

The backtracking search proceeds by identifying the column with the fewest intersecting rows, then trying each row in turn, deleting that row and also deleting all columns that row touches, and recursively searching for new steps until all columns are deleted (at which point a solution to the exact-cover problem is found).

### But this isn't an exact cover problem

That's right, we're looking for an *almost*-exact cover, and Algorithm X really takes advantage of the symmetries involved in seeking an exact cover. So I modified the problem spec slightly, including an "appendix" of all 26 one-letter words, and adding a special case in Algorithm X that if the algorithm chooses to make use of any of the words from the appednix, the entire appendix of one-letter words is immediately deleted (and then later undeleted in reverse order, of course, if that choice is backtracked).

## Why is it fast?

* After initialization, the solution algorithm performs no heap allocations.
* Everything stays in CPU cache. The state of the dancing links algorithm is less than 140kB.
* The algorithm is very aggressive about pruning.
* The heuristic of always branching only on the columns with the fewest nodes keeps the search tree dramatically smaller.
* Knuth is a genius and his optimizations are really good.
* Zero-overhead abstraction with Rust.
