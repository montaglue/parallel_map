# Parallel map

## Description
This is Simon Litvinskii test task.

## Task
Implement basic function to split some generic computational work between threads. Split should occur only on some threshold - if computational work (input length) is shorter than this threshold, no splitting should occur and no threads should be created.

You get as input: 

1. Vec<T>
2. Function f(t: T) -> R


Threshold can be just constant. 

You should return:
1. Up to you, but probably some Vec of the same length as input(1)

Code should be published on github.


There should be some tests in repository

## Important things
1. Output type must be Clone, I found out the solution without this bound, but it shares data between threads, and synchronization cost would be too big.
1. Benchmarks, now I didn't make any benchmarking. If I should please write me about it.
1. Dev-dependencies - tokio is really big and the solution itself doesn't use it. To speed up build I put it in dev-dependencies.
1. Using map in map_chunk. I prefer don't use it because it takes a function, not a reference. That makes me bound F also a Clone trait, but I don't want that.

