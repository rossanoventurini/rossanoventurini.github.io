---
layout: post
title:  Sweep Line Algorithm
date: 2023-10-9 7:01:00
description: The Sweep Line Algorithm is a powerful paradigm for solving a variety of problems in computational geometry. In these notes, we employ it to address problems both on a line and in a plane, providing Rust implementations for their solutions.
tags: rust, algorithms, data-structures
categories: notes
thumbnail: assets/img/SweepLine/MaxIntervalOverlaps_sweep.svg
giscus_comments: true
---
The *Sweep Line Algorithm* is an algorithmic paradigm used to solve a lot of problems in computational geometry efficiently. The sweep line algorithm can be used to solve problems on a line or on a plane.

<br>
#### Maximum Number of Overlapping Intervals

Let's start the description of this paradigm with a problem on a line.

*We are given a set of $$n$$ intervals $$[s_i, e_i]$$ on a line.*

*We say that two intervals $$[s_i, e_i]$$ and $$[s_j, e_j]$$ overlaps if and only if their intersection is not empty, i.e., if there exist at least a point $$x$$ belonging to both intervals.*

*The goal is to compute the maximum number of overlapping intervals.*

For example, consider the set of intevals in the figure.

<div class="row mt-3">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/SweepLine/MaxIntervalOverlaps.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

In this example, we have a set of $$10$$ intervals. The maximum number of overlapping intervals is $$5$$ (at positions $$3$$ and $$4$$).

The sweep line algorithm employs an imaginary *vertical line* sweeping over the x-axis.
As it progresses, we maintain a running solution to the problem at hand.
The solution is updated when the vertical line reaches certain key points where some *event* happen.
The type of the event tells us how to update the current solution.

To apply this paradigm to our problem, we let the sweep line move from left to right and stop at the beginning or the end of the intervals. These are the important points at which an event occurs: new intervals start or end.
We also maintain a counter which keeps track of the number of intervals that are currently intersecting the sweep line, along with the maximum value reached by the counter so far.
For each point, we first add to the counter the number of intervals that begin at that point, and then we subtract the number of intervals that end at that point.

The figure below shows the points touched by the sweep line and the values of the counter.

<div class="row mt-3">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/SweepLine/MaxIntervalOverlaps_sweep.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

Note that the sweep line touches only points on the x-axis where an event occurs. For example, points $$1$$ and $$6$$ are not taken into consideration. This is important because the number of considered points, and thus the time complexity, is proportional to the number of intervals and not to the size of the x-axis.

Here is a Rust implementation. We represent each interesting point as a pair consisting of the point and the kind, which is either `begin` or `end`. Then, we sort the vector of pairs in increasing order. Finally, we compute every state of the counter and its largest value. The correctness of the solution is based on a specific detail in the sorting step: since `begin` is considered smaller than `end`, if two points are the same, we first have pairs with `begin` and then pairs with `end`.

```rust
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Event {
    Begin,
    End,
}

pub fn max_overlapping(intervals: &[(usize, usize)]) -> usize {
    let mut pairs: Vec<_> = intervals
        .iter()
        .flat_map(|&(b, e)| [(b, PointKind::Begin), (e, PointKind::End)])
        .collect();

    pairs.sort_unstable();

    pairs
        .into_iter()
        .scan(0, |counter, (_, kind)| {
            if kind == Event::Begin {
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
Let's tackle a second problem to apply the sweep line paradigm to a two-dimensional problem.

*We are given a set of $$n$$ points in the plane.*

*The goal is to find the closest pair of points in the set. The distance between two points $$(x_1, y_1)$$ and $$(x_2,y_2)$$ is the Euclidian distance $$d((x_1,y_1), (x_2,y_2)) = \sqrt{(x_1-x_2)^2 +(y_1-y_2)^2}$$.*

A brute force algorithm calculates the distances between all possible pairs of points, resulting in a time complexity of $$\Theta(n^2)$$.


A faster algorithm employs the sweep line paradigm. We start by sorting the points in increasing order of their x-coordinates. We keep track of the shortest distance, denoted as $$\delta$$, seen so far. Initially, $$\delta$$ is set to the distance between an arbitrary pair of points.

We use a vertical sweep line to iterate through the points, attempting to improve the current shortest distance $$\delta$$. 
Consider the point $$p = (x, y)$$ just reached by the vertical sweep line. We can improve $$\delta$$ if the closest point *to the left* of $$p$$ has a distance smaller than $$\delta$$. If such a point exists, it must have an x-coordinate in the interval $$[x - \delta, x]$$, as it is to the left of $$p$$, and a y-coordinate in the interval $$[y - \delta, y + \delta]$$.

The figure below shows the rectangle within which this point must lie. We have a fact that, at a first glance, may seem quite surprising: *there can be at most $$6$$ points within the rectangle*. The $$6$$ circles within the perimeter of the rectangle represent points that are at distance exactly $$\delta$$ apart from each other. See  the *Section 5.4 of Algorithm Design by Kleinberg and Tardos* for a proof of this fact.

For our purposes, a slightly weaker result is sufficient, which states that *the rectangle contains at most $$8$$ points*.

To understand why, consider the $$8$$ squares in the figure above. Each of these squares, including its perimeter, can contain at most one point.
Assume, for the sake of contradiction, that a square contains two points, denoted as $$q$$ and $$q'$$. The distance between $$q$$ and $$q'$$ is smaller than $$\delta$$. If point $$q'$$ exists, it would have already been processed by the sweep line because it has an x-coordinate smaller than that of $$p$$. However, this is not possible, because otherwise the value of $$\delta$$ would be smaller than its current value.

<div class="row mt-3">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/SweepLine/ClosestPair.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

Now that we have the intuition of the solution, let's add more details.
The algorithm maintains a BST with points sorted by their y-coordinates. When we process 
point $$p=(x,y)$$, we iterate over the points with y-coordinates in the interval $$[y-\delta, y+\delta]$$.
If the current point has a $$x$$-coordinate smaller than $$x-\delta$$, we remove this point from the set. It  will be never useful anymore. Otherwise, we compute its distance with $$p$$ and update $$\delta$$ if needed. Before moving the sweep line to the next point, we insert $$p$$ in the set.

What is the complexity of this algorithm? Identifying the range of points with the required y-coordinates takes $$\Theta(\log n)$$ time. Iterating over the points in this range takes constant time per point and removing one of them takes $$\Theta(\log n)$$ time.

How many points do we need to iterate over? There can be at most $$6$$ points that have an x-coordinate greater than or equal to $$x-\delta$$ and therefore survive. On the other hand, there can be many points with smaller x-coordinates. However, since each point is inserted and subsequently removed from the set at most once during the execution of the algorithm, the cost of dealing with all these points is at most $$\Theta(n \log n)$$.

The following is a Rust implementation of this algorithm. There are two differences from the above description. First, we compute the squared Euclidean distance. This way, we avoid the computation of the square root, which is slow and results in a floating-point value
The second difference is that **we swap the roles of x and y**. Therefore, we process the points by ascending y-coordinate and use a horizontal sweep line.
This is easier to implement in Rust. Indeed, with the original approach, we would need to insert points into a [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) ordered by y-coordinate, which is the second component of the pair. This ordering is not possible with a `BTreeSet` unless we create a wrapper for a point that implements the required behavior for a comparison. Instead, if we swap the roles of x and y, the ordering by y is only required during the sorting step, which can be customized using the [`sort_unstable_by_key`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable_by_key) method.


```rust
pub fn distance_squared(p: (i64, i64), q: (i64, i64)) -> i64 {
    (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)
}

use std::collections::BTreeSet;
use std::ops::Bound::Included;

// Returns the (squared) Euclidean distance between the closest pair of 
// points in `points`
pub fn closest_pair(points: &mut [(i64, i64)]) -> Option<i64> {
    if points.len() < 2 {
        return None;
    }

    points.sort_unstable_by_key(|p| (p.1, p.0)); // sort by y

    let min_y = points[0].1;
    let max_y = points.last()?.1;

    let mut delta = distance_squared(points[0], points[1]);

    let mut set: BTreeSet<(i64, i64)> = BTreeSet::new();
    for &point in points.iter() {
        // Search by x and select the points with too small y-coordinate that we remove
        // to not touch them again in the future
        let to_delete: Vec<_> = set
            .range((
                Included(&(point.0 - delta, min_y)),
                Included(&(point.0 + delta, max_y)),
            ))
            .filter(|p| p.0 - delta >= point.0)
            .cloned()
            .collect();

        // Remove those points
        for p in to_delete {
            set.remove(&p);
        }

        // Search again and compute the distances with survived points.
        // Update delta if needed.
        delta = set
            .range((
                Included(&(point.0 - delta, min_y)),
                Included(&(point.0 + delta, max_y)),
            ))
            .fold(delta, |acc, &p| acc.min(distance_squared(point, p)));

        set.insert(point);
    }

    Some(delta)
}
```

<br>
#### Further Readings and Exercises
- [USACO Guide](https://usaco.guide/plat/sweep-line?lang=cpp)
- [Top Coder](https://www.topcoder.com/thrive/articles/Line%20Sweep%20Algorithms)
- [Fortune's algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm)
- [Check if all the integers in a range are covered](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/)
- [Cow Steeplechase II](http://www.usaco.org/index.php?page=viewproblem2&cpid=943)

These notes are for the [*"Competitive Programming and Contests"*](/rossano/competitive/) course at Università di Pisa.