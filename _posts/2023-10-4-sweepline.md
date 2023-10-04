---
layout: post
title:  Sweep Line Algorithm
date: 2023-10-4 4:01:00
description: Notes for the course Competitive Programming and Contests at University of Pisa
tags: rust, algorithms
categories: notes
thumbnail: assets/img/SweepLine/MaxIntervalOverlaps_sweep.svg
giscus_comments: true
---
The *Sweep Line Algorithm* is an algorithmic paradigm used to solve a lot of problems in computational geometry efficiently. The sweep line algorithm can be used to solve problems on a line or on a plane.

Let's start the description of this paradigm with a problem on a line.

<br>
#### Maximum number of overlapping intervals

*We are given a set of $$n$$ intervals $$[s_i, e_i]$$ on a line.*

*We say that two intervals $$[s_i, e_i]$$ and $$[s_j, e_j]$$ overlaps if and only if their intersection is not empty, i.e., if there exist at least a point $$x$$ belonging to both intervals.*

*The goal is to compute the maximum number of overlapping intervals.*

For example, consider the set of intevals in the figure.

<div class="row mt-3">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/SweepLine/MaxIntervalOverlaps.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

In this example, we have a set of $$10$$ intervals. The maximum number of overlapping intervals is $$5$$ (at position $$4$$).

The sweep line algorithm employs an imaginary *vertical line* sweeping over the x-axis.
As it progresses, we maintain a running solution to the problem at hand.
The solution is updated when the vertical line reaches certain key points where some *event* happen.
The type of the event tells us how to update the current solution.

To apply this paradigm to our problem, we let the sweep line move from left to right and stop at the beginning or the end of the intervals. These are the important points at which an event occurs: new intervals start or end.
We also maintain a counter which keeps track of the number of intervals that are currently intersecting the sweep line, along with the maximum value reached by the counter so far.
For each point, we first add to the counter the number of intervals that begin at that point, and then we subtract the number of intervals that end at that point."

<div class="row mt-3">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/SweepLine/MaxIntervalOverlaps_sweep.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

Note that the sweep line touches only points on the x-axis where an event occurs. For example, points $1$ and $6$ are not taken into consideration. This is important because the number of considered points, and thus the time complexity, is proportional to the number of intervals and not to the size of the x-axis.

Here is a Rust implementation. We represent each interesting point as a pair consisting of the point and the kind, which is either `begin` or `end`. Then, we sort the vector of pairs in increasing order. Finally, we compute every state of the counter and its largest value. The correctness of the solution is based on a specific detail: since `begin` is considered smaller than `end`, if two points are the same, we first have pairs with `begin` and then pairs with `end`.

```rust
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum PointKind {
    Begin,
    End,
}

pub fn max_overlapping(intervals: &[(usize, usize)]) -> usize {
    let mut pairs: Vec<_> = intervals
        .iter()
        .map(|&(b, e)| [(b, PointKind::Begin), (e, PointKind::End)])
        .flatten()
        .collect();

    pairs.sort_unstable();

    pairs
        .into_iter()
        .scan(0, |counter, (_, kind)| {
            if kind == PointKind::Begin {
                *counter += 1;
            } else {
                *counter -= 1;
            }
            Some(*counter)
        })
        .max()
        .unwrap()
}
```

<br>
#### Closest Pair of Points

In the second problem we use the sweep line paradigm on a set of 2D points.

*We are given a set of $n$ points in the plane.*

*The goal is to find the closest pair of points in the set. The distance between two points $$(x_1, y_1)$$ and $$(x_2,y_2)$$ is the Euclidian distance $$d((x_1,y_1), (x_2,y_2)) = \sqrt((x_1-x_2)^2 +(y_1-y_2)^2)$$.*
 
We start by sorting the points in increasing order of their x-coordinate. We keep track of the shortest distance $$\delta$$ seen so far. Initialy $$\delta$$ is the distance between an arbitrary pair of points.
We use a vertical sweep line to iterate through them, trying to improve the current shortest distance $\delta$. When we process the point $p=(x,y)$, we consider the window with the y-coordinates of all points with x-coordinate in the interval $[x-\delta, x]$. This is because any point with a smaller x-coordinate has a distance with $p$ which is surely larger than $\delta$. Furthermore, by a similar argument, we can consider only point with y-coordinate in the range $[y-\delta, y+\delta]$.

Observe that each point is inserted and removed from the set at most once, so the algorithm runs in $\Theta(n\log n)$ time.

- [https://en.wikipedia.org/wiki/Sweep_line_algorithm]
- [https://leetcode.com/discuss/study-guide/2166045/line-sweep-algorithms]
- [https://www.geeksforgeeks.org/closest-pair-of-points-using-sweep-line-algorithm/]
- [https://www.youtube.com/watch?v=tgQ3nfemjjQ&t=2474s]
- [https://usaco.guide/plat/sweep-line?lang=cpp]

#### Exercises 
- [Check if all the integers in a range are covered](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/)