[![Build Status](https://travis-ci.org/SergeyTsaplin/rust-algorithms.svg)](https://travis-ci.org/SergeyTsaplin/rust-algorithms)
[![Coverage Status](https://coveralls.io/repos/github/SergeyTsaplin/rust-algorithms/badge.svg)](https://coveralls.io/github/SergeyTsaplin/rust-algorithms)

# The Algorithms - Rust

Some base algorithms of Computer Science writen in Rust. It's not real project. Just for education and fun.

All actual documentation and example you can always find on the [Project page](http://sergeytsaplin.com/rust-algorithms/)

## Project Structure

All algorithms implemetations placed in [`src`](https://github.com/SergeyTsaplin/rust-algorithms/tree/master/src) folder.

### Sorts

#### Bubble Sort

![alt text][bubble-image]

[Implementation](https://github.com/SergeyTsaplin/rust-algorithms/blob/master/src/sorts/bubble_sort.rs)

From [Wikipedia][bubble-wiki]: Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares each pair of adjacent items and swaps them if they are in the wrong order. The pass through the list is repeated until no swaps are needed, which indicates that the list is sorted.

__Properties__
* Worst case performance: `O(n^2)`
* Best case performance: `O(n)`
* Average case performance: `O(n^2)`

__View the algorithm in [action][bubble-toptal]__

#### Insertion Sort

![alt text][insertion-image]

[Implementation](https://github.com/SergeyTsaplin/rust-algorithms/blob/master/src/sorts/insertion_sort.rs)

[Wikipedia][insertion-wiki]: Insertion sort is a simple sorting algorithm that builds the final sorted array (or list) one item at a time. It is much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.

__Properties__
* Worst case performance: `O(n^2)`
* Best case performance: `O(n)`
* Average case performance: `O(n^2)`

__View the algorithm in [action][insertion-toptal]__

### Tests

Functional tests placed in [`tests`](https://github.com/SergeyTsaplin/rust-algorithms/tree/master/tests) folder

## TODO

- [x] CI integration
- [x] Autodoc building
- [ ] Merge sort
- [ ] Selection sort
- [ ] Quick sort
- [ ] Binary search


[bubble-toptal]: https://www.toptal.com/developers/sorting-algorithms/bubble-sort
[bubble-wiki]: https://en.wikipedia.org/wiki/Bubble_sort
[bubble-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Bubblesort-edited-color.svg/220px-Bubblesort-edited-color.svg.png "Bubble Sort"

[insertion-toptal]: https://www.toptal.com/developers/sorting-algorithms/insertion-sort
[insertion-wiki]: https://en.wikipedia.org/wiki/Insertion_sort
[insertion-image]: https://upload.wikimedia.org/wikipedia/commons/7/7e/Insertionsort-edited.png "Insertion Sort"

