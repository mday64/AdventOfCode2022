For the HashMap used for cycle detection, simplify the key to just be a bunch of integers:
* Offset into input ("jet" sequence)
* Offset into rock sequence
* Heights of the 7 columns, relative to the highest (or lowest?) column

This means that instead of using an iterator and .cycle() for the jets and rocks,
we should just use a simple numeric index that we keep modulo the corresponding
length.

We should try to maintain the height of each column, not just the overall
height (of the tallest column in) the chamber.

Look into using the fxhash crate for a potentially faster hash of integers.

Define specific key and value types to make the code more self-documenting.

Should the "chamber" be a Vec of u8 (instead of u16)?  It might mean slightly
more checking to see if we would hit the chamber wall, in addition to checking
to see if we would hit existing, settled, rock.

Should a rock be represented as a Vec of points?  That's what's used in
[this Python solution](https://github.com/tmo1/adventofcode/blob/main/2022/17b.py)
and in [fasterthanlime's Rust port](https://fasterthanli.me/series/advent-of-code-2022/part-17).
A potential advantage is that we can order the points such that we can easily
find the left-most and right-most x-value to make it simple to detect collision
with the walls.  Note that the chamber becomes a set of points.
