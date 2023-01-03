# Simplification

Most locations in the input have zero flow rate.  There is no point in opening their valves.  Only 15 valves have non-zero flow.

The problem can be simplified to figuring out the optimal order to turn on valves with non-zero flow.  We can pre-calculate the cost to move between these valves by using an all-pairs shortest paths algorithm.  Unfortunately, it's not as simple as trying all possible orderings.

In some cases, moving from one valve to another might pass through a location with a (non-zero flow) valve.  We have the option to either take time to open that intermediate valve (which will reduce the amount of time our destination valve is open), or just pass through on our way to the destination.

# Path Finding

I'm trying to figure out an algorithm for finding a path with *maximum* score (not minimum cost).  I'd like to do better than exploring all possible options.

## What is "cost", anyway?

Dijkstra'a Algorithm and the A* Algorithm assume non-negative costs to move between nodes/states, and wants to minimize that cost.  Is there a way of framing the problem so that instead of maximizing flow, we can somehow minimize a cost that directly corresponds to maximizing flow?

I'm thinking of "cost" as flow that didn't happen (kind of like opportunity cost) as the result of moving to the next state.  Let's assume that zero cost is when all valves open at time t=0.  Moving to a valve and opening it costs us the flow from all closed valves times the time to move and open the valve.

That seems wrong, or at least inadequate.  "Cost" depends entirely on distance to the next valve, and doesn't consider the effect of future flow from the to-be-opened valve.  Can we subtract the future flow from that valve from the cost?  I don't think so, since that seems to be double-counting when we examine the next set of neighbors.

We need to be able to count that future flow...

## What is "done", anyway?

BFS, Dijkstra and A* all stop when they've found a path to the finish; when they've hit a state that is "done."  In my original solution, this was a problem.  I was "done" when there was no time left to open any more valves, or there were no valves left to open.  But this, combined with my negative costs, meant that it tried to stop once it found any path that reached a "done" state, without trying any others.

Using the opportunity cost idea, I think "done" still means running out of time.  The key is that *there is a cost* to get to the done state: the lost flow from the remaining closed valves.  As a bonus, I think that makes the solution to part 1 fall simple to calculate: ideal flow (all valves open at time t=0) minus the flow opportunity cost of the optimum path.

## Pruning

Dijkstra's Algorithm and the A* Algorithm improve upon breadth-first search by being able to prune (not examine) large portions of the graph.  When trying to minimize cost, they can compare a given node/state's cost so far to the best known solution.  If the cost so far is larger, there is no possible way any path resulting from that node/state could be optimal.  (Note: this assumes non-negative costs!)

When finding a maximum, can we prune by calculating an upper bound on the rest of the path's score?  In this problem, I'm thinking of taking the still-closed valves, and pretend that we close them in order from largest to smallest flow.  Since we don't know the travel times, we can come up with a bound by assuming that it takes 1 minute to travel to the next valve, 1 minute to open it, and then flow commences.  We can tighten the bound slightly by keeping track of time remaining, and not counting potential score from valves we couldn't get to in the remaining time.

## Hueristic

It would be helpful to have a heuristic that guides us in the order that we explore neighboring states, so that we are more likely to find the optimal path early, and more quickly prune out paths that can't possibly beat the best known one.

My first instinct towards a heuristic is the potential score that the neighbor would add if we open its valve.  That is (time_remaining - time_to_travel - time_to_open_valve) * flow_rate.  We should prefer to try neighbors in order from largest to smallest potential score.

## Greed

Greed can be good.  A greedy approach to finding the optimal path would have us open valves in order of maximum future flow (as in the Hueristic section).  But is a greedy approach going to be optimal?  (Here's where stopping to open an intermediate valve might be helpful.  If the future flow of that intermediate valve is more than the flow rate of the destination valve for one minute, then it seems to make sense to stop and open the intermediate valve.)
