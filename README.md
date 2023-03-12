# Code Complexity

- how does pylint do it?
- number of allocations

## Cyclomatic complexity
number of branches

## knots
re-ordering functions to reduce knots: https://www.geeksforgeeks.org/complexity-metrics/ (early return instead of if else reduces knots?)
Can we also get a heuristic for if funcs should be moved into a separate file?

## Big O
pitfalls of big O notation
how to measure your program's big O (everyone knows nested for loops, but what about logn bits? or recursive algorithms)

## implementation
use syn crate or treesitter

and report (maybe via lsp)

## links
	https://en.wikipedia.org/wiki/Software_metric
    https://en.wikipedia.org/wiki/Programming_complexity 

