---
layout: post
title: The Power of Prefix Sums 
date: 2023-10-10 7:01:00
description: Prefix sums offer an elegant and efficient solution to a variety of problems. In this notes, we showcase several such problems and provide their solutions with Rust implementations.
tags: rust, algorithms, data-structures
categories: notes
thumbnail: assets/img/prefixsums/Prefixsums_1.svg
giscus_comments: true
---
*Prefix sums*, also known as cumulative sums or cumulative frequencies, offer an elegant and efficient way to solve a wide range of problems that involve querying cumulative information about a sequence of values or elements.

The essence of prefix sums lies in transforming a given array of values into another array, where each element at a given index represents the cumulative sum of all preceding elements in the original array.

To be more formal, let's assume we have an array $$A[1,n]$$ of values, and our objective is to support the query `range_sum(i,j)`, which returns the sum of the values in the subarray $$A[i..j]$$.

For example, suppose you have an array $$A[1,8]$$ with values: [2, 4, 1, 7, 3, 0, 4, 2]. The query `range_sum(2, 6)` equals $$4+1+7+3+0 = 15$$.

These queries can be solved in constant time by maintaining the prefix sum array. This array $$P[1,n]$$ stores, at any position $$i$$, the sum of the values in $$A$$ up to the $$i$$th position. In other words, $$P[i] = \sum_{k=1}^i A[k]$$.

The arrays $$A$$ and $$P$$ are shown in the figure below.

<div class="row mt-3 ">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/prefixsums/Prefixsums_1.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

Armed with $$P$$, a `range_sum(i,j)` query is resolved by calculating $$P[j]-P[i-1]$$.

Continuing the example shown in the figure above, `range_sum(2, 6)` is $$P[6] - P[1] = 17 - 2 = 15$$.

<br>
#### Prefix Sums in Rust
In Rust, the combinator  [`scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) can produce the prefix sums (and much more) from an iterator.

`scan` is an iterator adapter that bears similarity to [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold). Similar to `fold`, `scan` maintains an internal state, initially set to a seed value, which is modified by a closure taking both the current internal state and the current element from the iterator into account.

The distinction between `scan` and `fold` is that the former produces a new iterator with all the states taken by its internal state, whereas the latter only returns the value of the final internal state.

The following code snippet illustrates how to employ `scan` for computing prefix sums.

```rust
let a = vec![2, 4, 1, 7, 3, 0, 4, 2];

let psums = a
    .iter()
    .scan(0, |sum, e| {
        *sum += e;
        Some(*sum)
    })
    .collect::<Vec<_>>();

assert!(psums.eq(&vec![2, 6, 7, 14, 17, 17, 21, 23]));
```

<br>
#### Applications of Prefix Sums
Range sum queries are exceptionally useful for solving a variety of other problems involving other kind of range queries. We present here solutions to three problems from CodeForces, which serve as examples of the power of prefix sums. 

Below, you'll find links to these problems if you'd like to attempt them yourself before reading their solutions.

- [Ilya and Queries](http://codeforces.com/problemset/problem/313/B?locale=en)
- [Number of Ways](http://codeforces.com/problemset/problem/466/C?locale=en)
- [Little Girl and Maximum](http://codeforces.com/problemset/problem/276/C?locale=en)


<br>
##### Ilya and Queries

*We have a string $$s=s_1s_2 \ldots s_n$$ consisting only of characters $$a$$ and $$b$$ and we need to answer $$m$$ queries.*

*Each query $$q(l, r)$$, where $$1 \leq l < r \leq n$$, asks for the number of positions $$i \in [l, r]$$ such that $$s_i = s_{i+1}$$.*

Let's consider an example to better illustrate this problem.

Given string $$s = aabbbaaba$$. Consider the query $$q(3, 6)$$. We are interested in the substring $$bbba$$. So, the answer for this query is $$2$$ because there are three positions followed by the same symbol, namely position $$1$$, $$2$$, and $$4$$ in the substring.

The idea is that of computing the binary vector $$B[1,n]$$ such that $$B[i]=1$$ if $$s_i == s_{i+1}$$, $$0$$ otherwise. This way, the answer to the query $$q(l,r)$$ is $$\sum_{i=l} ^{r-1} B[i]$$. Thus, each query can be solved in constant time by computing prefix-sums on vector $$B$$.

For example, the binary vector $$B$$ for the string $$s = aabbbaaba$$ is [1, 0, 1, 1, 0, 1, 0, 0, 0]. Its prefix sum array $$P$$ is [1, 1, 2, 3, 3, 4, 4, 4, 4]. Therefore, the query $$q(3,6) = P[5]-P[2] = 3-1 = 2$$.

The Rust implementation is as follows.

```rust
#[derive(Debug)]
struct Ilya {
    psums: Vec<usize>,
}

impl Ilya {
    pub fn new(s: &str) -> Self {
        let psums = s
            .as_bytes()
            .windows(2)
            .map(|w| if w[0] == w[1] { 1usize } else { 0usize })
            .scan(0, |sum, e| {
                *sum += e;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        Self { psums }
    }

    // Queries use 0-based indexing
    pub fn q(&self, i: usize, j: usize) -> usize {
        assert!(i < j);
        assert!(j <= self.psums.len());

        self.psums[j - 1] - if i != 0 { self.psums[i - 1] } else { 0 }
    }
}
```


<br>
##### Little Girl and Maximum
*We are given an array $$A[1,n]$$ and a set $$Q$$ of $$q$$ queries. Each query is a range sum query $$i,j$$ which returns the sum of elements in $$A[i..j]$$.*

*The goal is to permute the elements in $$A$$ in order to maximize the sum of the results of the queries in $$Q$$.*

The main observation is that if we want to maximize the sum, we have to assign the largest values to the most frequently accessed entries. Thus, the solution consists of sorting both $$A$$ by descending values and the indexes of $$A$$ by descending frequency of access and pairing them in this order. Therefore, once we have computed the frequencies, the solution takes $$\Theta(n\log n)$$ time.

Thus, we are left with the problem of computing access frequencies. In other words, we want to compute the array $$F[1,n]$$, where $$F[i]$$ is the number of times the index $$i$$ belongs to a query of $$Q$$. Computing this vector by updating every single entry in $$F$$ for each query takes $$O(nq)$$ and, thus, is clearly infeasible.

We require a faster algorithm to compute these frequencies. One possible solution involves using the [sweep line algorithm](blog/2023/sweepline/). Since the queries represent intervals, and calculating the frequencies equates to counting the number of overlapping intervals at each position, we can employ an approach similar to the one used in solving the *Maximum Number of Overlapping Intervals* problem, as detailed in these [notes](blog/2023/sweepline/).

This solution has a time complexity of $\Theta(q\log q)$, due to the comparison-based sorting of interval endpoints. Since the endpoints in our problem have a maximum value of $n$, we can optimize the solution to run in $\Theta(q)$ using counting sort. However, there exists an alternative solution based on prefix sums, which is much simpler to implement.

The main idea of this alternative solution is to construct an array $$U[1\ldots n]$$ such that its prefix sums are equal to our target array $$F$$. Interestingly, we need to modify just two entries of $$U$$ to account for a query in $$Q$$.

Initially, all the entries of $$U$$ are set to $$0$$. For a query $$\langle l, r \rangle$$, we add $$1$$ to $$U[l]$$ and subtract $$1$$ from $$U[r+1]$$. This way, the prefix sums are as follows:

- Unchanged for indexes less than $$l$$.
- Increased by one for indexes in $$[l, r]$$.
- Unchanged for indexes greater than $$r$$.

Therefore, the prefix sum of $$U$$ up to $$i$$ equals $$F[i]$$. This algorithm takes $$O(q+n)$$ time.

Here's the Rust implemetation.

```rust
// We assumes queries are 0-based indexed
pub fn little_girl(a: &[i64], q: &[(usize, usize)]) -> i64 {
    if a.is_empty() {
        return 0;
    }

    let mut u = vec![0i64; a.len()];

    for &(l, r) in q {
        assert!(l <= r);
        assert!(r < u.len());

        u[l] += 1;
        if r + 1 < u.len() {
            u[r + 1] -= 1;
        }
    }

    let mut f = u
        .iter()
        .scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    // we sort both f and a in decreasing order, nothing changes
    f.sort_unstable();
    let mut a_sorted = a.to_vec();
    a_sorted.sort_unstable();

    a_sorted
        .iter()
        .zip(f)
        .fold(0, |result, (value, freq)| result + value * freq)
}
```

<br>
##### Number of Ways
*Given an array $$A[1,n]$$, count the number of ways to split the array into three contiguous parts so that they have the same sums.*

More formally, you need to find the number of such pairs of indices $$i$$ and $$j$$ ($$2 \leq i \leq j \leq n-1$$) such that:

$$\sum_{k=1}^{i-1} A[k] = \sum_{k=i}^{j} A[j] = \sum_{k=j+1}^n A[k]$$

For the solution, let $$S$$ be the sum of the values in the array. If $$3$$ does not divide $$S$$, we conclude that the result is zero. Otherwise, we compute an array $$C$$ that stores, at position $$i$$, the number of suffixes of $$A[i\ldots n]$$ that sum to $$\frac{S}{3}$$. 
Then, we scan $$A$$ from left to right to compute the prefix sums. Every time the prefix sum at position $$i$$ is $$\frac{S}{3}$$, we add $$C[i+2]$$ to the result. This is because the part $$A[1..i]$$ sums to $$S/3$$ and can be combined with any pair of parts of $$A[i+1..n]$$ where both parts sums to $$S/3$$. Since the values in $$A[i+1..n]$$ sum to $$2/3 S$$, the number of such pairs is the number of suffixes that sum to $$S/3$$ in $$A[i+2..n]$$.
Indeed, if one of this suffix sums to $$S/3$$, say $$A[j..n]$$, then we are sure that $$A[i+1, j-1]$$ sums to $$S/3$$.

Here's a Rust implementation.

```rust
pub fn number_of_ways(a: &[i64]) -> usize {
    let sum: i64 = a.iter().sum();

    if sum % 3 != 0 {
        return 0;
    }

    let target = sum / 3;
    let mut c: Vec<_> = a
        .iter()
        .rev()
        .scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        })
        .scan(0, |counter, sum| {
            if sum == target {
                *counter += 1usize
            };
            Some(*counter)
        })
        .collect();

    c.reverse();

    let mut result = 0;
    let mut sum = 0;
    for (i, &v) in a[..a.len() - 2].iter().enumerate() {
        sum += v;
        if sum == target {
            result += c[i + 2];
        }
    }

    result
}
```
<br>
#### Exercises
- [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)
- [Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)
- [Good Subarrays](https://codeforces.com/contest/1398/problem/C)
- [Running Miles](https://codeforces.com/contest/1826/problem/D)

These notes are for the [*"Competitive Programming and Contests"*](/rossano/competitive/) course at Università di Pisa.