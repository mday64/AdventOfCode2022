# lib.rs
Enhance the RangeSet type to be a more general set.
* Allow inserting or removing individual values
* Allow membership tests (ranges or individual values)
* Intersection, Union, Difference
* Collect from iterator (of ranges or individual values)

# Day 15
Fixed distance using Manhattan distance results in a square that is rotated by 45º from the axes.  This might be useful for better solutions, especially for part 2.

## Edges (Part 2)
I think that the solution must be a distance of one more than a sensor's radius, for four sensors.  That is, I think it is bounded by radius squares from four sensors.

One possibility is to iterate through sensors, then iterate Cartesian points one beyond each edge, looking for a point that is outside of all sensor ranges.  Points along an edge are just a line in Cartesian space, and it should be easy to constrain them to the problem's bounding box.  This technique could significantly reduce the number of points to consider.

Likewise, we could look for an intersection of bounding edges of two sensors.  In Cartesian space, this is a simple line intersection.  The solution must be in the set of these intersection points.

## Half planes (Part 2)
Because the sensor ranges are squares, the area outside of the squares is a union of half-planes (a half-plane bordering an edge of the square and pointing away from it).  If there is an efficient way to intersect half planes, then the solution is the intersection of one half-plane from each sensor.  If there are S sensors, then that's 4**S combinations.

If we think about rotating the sensor squares so that their edges were vertical and horizontal, those half-planes become rectangles where three sides have infinite coordinates.  Intersections between rectangles remain rectangles (or empty), and possibly change an infinite coordinate to a finite one.

# Day 16
## Better Searching
There's got to be a better algorithm for maximizing flow rate.  Typical path finding tries to minimize cost, and can stop examining a solution once cost-so-far exceeds the best solution found so far ("pruning").  Is there a way to prune other solutions based on distance to nearest valve times sum of remaining flow rates?  Some other way to prune?

Is there a dynamic programming solution akin to Floyd-Warshall all-pairs shortest paths?  (Thinking about how it selects an intermediate node and finding the best known path through that node.  Don't Dijkstra and A* do something similar about updating "best so far"?)

Seems like there is an opportunity for a heuristic here.  Try next valves with higher flow rates first (or perhaps (time_remaining - time_to_valve) * flow_rate).

Would depth-first or breadth-first search be best?  My intuition says depth-first, especially in combination with a heuristic.  Plus, that's probably far more space efficient.

A* algorithm does pruning and uses a heuristic.  Could it be adapted to find a maximal solution?  Would negative costs work?  The problem is that A* (and Dijkstra?) assume that once you've found any solution, it must be the best (is that the assumption about non-negative edge weights?).  My existing solution was forced to examine all potential solutions and find the maximum.  It couldn't even "forget" non-optimal solutions as it found them in order to save memory (though it probably wouldn't have saved much time).

## Partitioning (Part 2)
Consider partitioning the set of "interesting" valves (the ones with non-zero flow rates) into two non-empty subsets.  Give one subset to the human, and one to the elephant.  Let them each find their own optimal solution, and add the resulting flows.  With 15 "interesting" valves in the full input, this is less than 2**15 partitions to consider.

For a given partition, we could find the optimal path for each of {human, elephant} in separate threads, for up to 2X speedup.  When trying multiple partitions, we could parallelize those, too.

Is there a way to order the partitions such that we start by trying partitions that are "more even"?  Perhaps trying to assign similar total flow rates?

# Day 19
It's SLOW!

I suspect the biggest part of the problem is that there are just an enormous number of states to examine.  Is there a way to prune choices (back to the problem of trying to find a maximal path)?  Is there some way to represent the problem as a shortest path?

Would a greedy solution be optimal?

Is there a way to recognize a pattern for how many of each robot you need, or should continue to make?  I'm guessing that the optimal solution is repeatedly making the same quantities of each robot for as long as time allows.

Would this be a good example of profiling?

Am I examining the same/equivalent states multiple times?  Maybe I should add a cache of visited states, and print out when it finds a duplicate...  No, dfs_reach() already does that for me.

Use a Breadth-First Search instead of Depth-First?  If I'm pruning branches that can't possibly do better than the current best solution, would BFS end up pruning more states than DFS?

Current solution has a choice at every time step to do nothing, and not build any bot, even if it was possible to build one.  Is this actually needed in any solution?  (Yes!)  Perhaps it would be better to decide on a type of bot to build, then advance time until that bot has been built.

This [reddit thread](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j1vj08v/?utm_source=share&utm_medium=web2x&context=3) talks about pruning paths where "theoretical production" can't possibly make as many geodes as the best known solution.  That theoretical production is all current geode robots producing for the remaining time, and assuming you can make a new geode robot every minute, which then produces thereafter.

[Here](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j1q5l05/?utm_source=share&utm_medium=web2x&context=3) is a Rust solution using Depth First Search with branch and bound, that purports to be extremely fast.  (Is this just another name for pruning demonstrably worse paths/states?)

[This Javascript solution](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j15jpqn/?utm_source=share&utm_medium=web2x&context=3) suggests pruning paths that yield geodes later than the earliest found in any solution.  Also, deciding which bot to build, and skipping time steps until you can actually build that bot.

A [Rust solution](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0vvtdt/?utm_source=share&utm_medium=web2x&context=3) suggests pruning a branch if it tries to create a robot of a given type, but it could have also created that robot in the previous state but it decided not to.  I think this is one way of handling accumulating resources to build an obsidian robot when you could have created an ore or clay robot.  I think it is an alternative to "pick the kind of robot you want to make, and advance time until you can make it."

[This Rust solution](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0vigd6/?utm_source=share&utm_medium=web2x&context=3) suggests that if you have reached the max number of robots of a given type, and you have more of that resource than you could use in a single turn, set the quantity of that resource to the maximum you could use.  This could potentially reduce the number of distinct states being cached.

Another, simple [Rust solution](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0xaaxn/?utm_source=share&utm_medium=web2x&context=3).  And [another](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0wzy3k/?utm_source=share&utm_medium=web2x&context=3).  And [yet another](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0vvzgz/?utm_source=share&utm_medium=web2x&context=3).  More [Rust](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0vt06q/?utm_source=share&utm_medium=web2x&context=3).  And [this one](https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0v1sul/?utm_source=share&utm_medium=web2x&context=3) might be pretty clean.

# Day 20
Investigate a data structure that makes finding, removing, and inserting an element faster than O(n).  Then revert back to the remove/insert way of moving an element.  Note: requires being able to get an item's index, and insert at index; whereas finding an element is "by value" (using the index created by the enumerate() method).

Current solution keeps the element's original index with the number value.  Does it make sense to separate those?  Would it be better to mix a Vec of the original indices, and have another Vec of the values?  Looking up a value in the mixed lists would require first looking in the Vec of indices, then using that to look up in the Vec of values.

# Day 21
For part 2, convert to an equality equation, and figure out how to solve for "humn" directly by back substituting and inverting operations as needed.

I think the branch (of "root") that contains "humn" can be simplified bottom-up to something of the form "m * humn + c".  The other branch will evaluate to a constant (call it "k").  Then you'd have m * humn + c = k.  Solving for humn: humn = (k - c) / m.

# Day 24
Rather than moving every blizzard by one for every call to `successors`, we can take advantage of their wrap-around behavior, plus the fact that every blizzard stays in its own row or column.  Adjust the coordinates so that the top row and left column of rock are both at -1.  That way, the interior of the rectangle has coordinates that go from 0..width and 0..height.  The position of a blizzard is just M+c % width, or M+c % height.

Further, if we keep track of the non-changing coordinate, we can search for only those blizzards that are within one row or column of our current position.  (I think this means keeping track of horizontal vs. vertical movements separately.)
