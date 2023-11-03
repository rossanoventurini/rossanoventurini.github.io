---
layout: post
title: Mo's Algorithm
date: 2023-10-31 06:01:00
description: The Mo's Algorithm is a powerful and efficient technique for solving a wide variety of range query problems. It becomes particularly useful for kind of queries where the use of a Segment Tree or similar data structures is not feasible.
tags: rust, algorithms, data-structures
categories: notes
thumbnail: assets/img/mos/Mos_1.svg
giscus_comments: true
---
The *Mo's Algorithm* is a powerful and efficient technique for solving a wide variety of range query problems. It becomes particularly useful for kind of queries where the use of a Segment Tree or similar data structures is not feasible. This typically occurs when the query is non-associative, meaning that the result of a query on a range cannot be derived by combining the answers of the subranges that cover the original range.

Mo's algorithm tipically achieves a time complexity of $$O((n+q)\sqrt{n})$$, where $$n$$ represents the size of the dataset, and $$q$$ is the number of queries.

<br>
#### A Difficult Problem 
Let's consider the following problem.

*We are given an array $$A[1, n]$$ of integers and our goal is to solve $$q$$ queries `power` . For a query `power`(l,r) we have to compute some value for the subarray $$A[l, r]$$. For each integer $$s$$ within this subarray, let $$K_s$$ represent the number of occurrences. The subarray's power is defined as the sum of the products $$s \cdot K_s \cdot K_s$$ for every positive integer $$s$$ that appears in the subarray.*

Our goal is to achieve a time complexity of $$\Theta((n+q)\sqrt{n})$$ to solve all the $$q$$ queries. This may appear quite challenging, and you might even wondering where the factor $$\sqrt{n}$$ is coming from.

For now, let's temporarily set aside the current problem and begin by introducing the Mo's algorithm with a simpler one. By the end of these notes, you'll be astonished at how straightforward the earlier problem becomes with the right algorithmic approach.

<br>
#### A Easier Problem
For many types of range queries, such as `RangeSum`, `RMQ`, `Distinct`, and others, there exist suitable data structures (like the Segment Tree) to answer queries efficiently and online.
Solving a query online means that the data structure answers the query as soon as it is presented, without any delay. However, for some more complex query types, there are no online-efficient data structures available.

For certain types of queries, the most we can hope for is an algorithm that operates efficiently only when dealing with a sufficiently large batch of queries. Consequently, the time complexity of an individual query is low only in an amortized sense. The Mo's algorithm is one of these strategies. It ensures that if the batch consists of $$q = \Omega(n)$$ queries, each query can be resolved in $$\Theta(\sqrt{n})$$ amortized time.

Consider now the following problem.

*We are given an array $$A[0,n-1]$$ consisting of colors, with each color represented by an integer within the range $$[0, n-1]$$. Additionally, we are given a set of $$q$$ range queries called `three_or_more`. The query `three_or_more`(l, r) aims to count the colors that occur at least three times within the subarray $$A[l, r]$$*

Let's begin by examining a straightforward algorithm that addresses a query `three_or_more(l, r)` by scanning the subarray $$A[l, r]$$. The algorithm maintains an array of `counters` to track the number of occurrences of each color within the query range. Whenever a color reaches three occurrences, the `answer` is incremented by one.


Below is a Rust implementation of this algorithm.

```rust
pub fn three_or_more_slow(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: Vec<usize> = vec![0; a.len()];
    let mut answers = Vec::with_capacity(queries.len());

    for &(l, r) in queries {
        let answer = a[l..=r].iter().fold(0, |ans, &color| {
            counters[color] += 1;
            if counters[color] == 3 {
                ans + 1
            } else {
                ans
            }
        });

        answers.push(answer);

        a[l..=r].iter().for_each(|&color| counters[color] = 0);
    }

    answers
}
```

After each query, it's essential to reset the vector of counters. In the above implementation, this reset is achieved using the code snippet `a[l..=r].iter().for_each(|&color| counters[color] = 0)`. What's noteworthy is that this method selectively resets only the counters associated with colors within the queried subarray. This approach ensures that the time spent on resetting is directly proportional to the size of the queried range, rather than the length of `counters`. Consequently, this gives a better running time when dealing with short queried subarrays. However, that this minor optimization doesn't change the worst-case time complexity: *the algorithm remains still very slooooooow*.

Indeed, it's evident that this algorithm has a time complexity of $\Theta(qn)$. The figure below illustrates an input that showcases the worst-case running time. We have $$n$$ queries. The first query range has a length of $$n$$ and spans the entire array. Then, the subsequent query ranges are each one unit shorter, until the last one, which has a length of one. The total length of these ranges is $$\Theta(n^2)$$, which is also the time complexity of the solution.

<div class="row mt-1">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/mos/Mos_1.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>


<br>
#### Mo's algorithm
Let's now introduce a different way to implementing the inefficent algorithm above. At first glance, this may appear to be just a more convoluted way of implementing the same strategy, seemingly offering no advantage in terms of worst-case running time. However, as we will see later on, we can achieve a  significantly improved time complexity just by strategically rearranging the queries.

Suppose we have just answered the query for the range $$[l', r']$$ and are now addressing the query for the range $$[l, r]$$. Instead of starting from scratch, we can update the answer and the counters by adding or removing the contributions of colors that are in the new query range but not in the previous one, or vice versa. Specifically, for the left endpoints, we must remove all the colors in $$A[l', l-1]$$ if $$l' < l$$, or we need to add all the colors in $$A[l, l'-1]$$ if $$l < l'$$. The same principle applies to the right endpoints $$r$$ and $$r'$$.

The Rust implementation below utilizes two closures, `add` and `remove`, to keep `answer` and `counters` updated as we adjust the endpoints.

```rust
pub fn three_or_more(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: Vec<usize> = vec![0; a.len()];
    let mut answers = Vec::with_capacity(queries.len());

    let mut cur_l = 0;
    let mut cur_r = 0; // here right endpoint is excluded
    let mut answer = 0;

    for &(l, r) in queries {
        let mut add = |i| {
            counters[a[i]] += 1;
            if counters[a[i]] == 3 {
                answer += 1
            }
        };

        while cur_l > l {
            cur_l -= 1;
            add(cur_l);
        }

        while cur_r <= r {
            add(cur_r);
            cur_r += 1;
        }

        let mut remove = |i| {
            counters[a[i]] -= 1;
            if counters[a[i]] == 2 {
                answer -= 1
            }
        };

        while cur_l < l {
            remove(cur_l);
            cur_l += 1;
        }

        while cur_r > r + 1 {
            cur_r -= 1;
            remove(cur_r);
        }

        answers.push(answer);
    }

    answers
}
```

The time complexity of this algorithm remains $\Theta(qn)$. However, we observe that a query now executes more quickly if its range significantly overlaps with the range of the previous query.

For example, the input in the figure above is no longer the worst-case for the new implementation. Conversely, it is actually a best-case as the implementation takes $\Theta(n)$ time. After spending linear time on the first query, any subsequent query is answered in constant time.

We also observe that this implementation is highly sensitive to the ordering of the queries. It is enough to modify the ordering of the queries, as shown in the figure below, to revert to quadratic time. In the example below, we rearrange the queries from the figure above to alternate between a long and a short query. With this ordering, the new implementation again takes $\Theta(n^2)$ time.

<div class="row mt-1">
    <div class="col-sm mt-3 mt-md-0">
        {% include figure.html path="assets/img/mos/Mos_2.svg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

These considerations lead to a question: *if we have a sufficient number of queries, can we rearrange them in a way that exploits the overlap between successive queries to gain an asymptotic advantage in the overall running time*?

Mo’s algorithm answers positively this question by providing a reordering of the queries such that the time complexity reduces to $$\Theta((q+n)\sqrt{n})$$. Thus, we shave a factor $$\sqrt{n}$$ when $$q=\Omega(n)$$.

The idea is to conceptually partition the array $$A$$ into $$\sqrt{n}$$ buckets, each with a size of $$\sqrt{n}$$, named $$B_1, B_2, \ldots, B_{\sqrt{n}}$$. A query belongs to bucket $$B_k$$ if and only if its left endpoint $$l$$ falls into the $$k$$-th bucket, which can be expressed as $$\lfloor l/\sqrt{n} \rfloor = k$$.

Initially, we group the queries based on their corresponding buckets, and within each bucket, the queries are solved in ascending order of their right endpoints.

We now analyze the time complexity of the above algorithm with this query reordering.

Let's concentrate on a specific bucket. As we process the queries in ascending order of their right endpoints, the index `cur_r` in the algorithm only increases from $$1$$ to $$n$$, moving a total of at most $$n$$ steps. On the other hand, the index `cur_l` can both increase and decrease but cannot move more than $$\sqrt{n}$$ steps per query. Thus, for a bucket with $$b$$ queries, the overall time to process its queries is $$\Theta(b\sqrt{n} + n)$$.

Summing up over all buckets, the time complexity becomes Θ(q√n + n√n), which results in Θ(√n) amortized time per query when m = Ω(n)."

Summing up over all buckets, the time complexity is $$\Theta(q\sqrt{n} + n\sqrt{n})$$, which results in $$\Theta(sqrt{n})$$ amortized time per query when $$m = \Omega(n)$$.

Here's a Rust implementation of the reordering process. We sort the queries by buckets, using their left endpoints, and within the same bucket, we sort them in ascending order of the right endpoints. We also have to compute a `permutation` to keep track of how the queries have been reordered. This permutation is essential for returning the answers to their original ordering.

```rust
pub fn mos(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    // Sort the queries by bucket and get the permutation induced by this sorting.
    // The latter is needed to permute the answers back to the original ordering
    let mut sorted_queries: Vec<_> = queries.iter().cloned().collect();
    let mut permutation: Vec<usize> = (0..queries.len()).collect();

    let sqrt_n = (a.len() as f64) as usize + 1;
    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt_n, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt_n, queries[i].1));

    let answers = three_or_more(a, &sorted_queries);

    let mut permuted_answers = vec![0; answers.len()];
    for (i, answer) in permutation.into_iter().zip(answers) {
        permuted_answers[i] = answer;
    }

    permuted_answers
}
```
<br>
#### The Difficult Problem Revisited
As I promised, the challenging problem introduced above no longer seems that hard. Just use Mo's algorithm.

#### Final Consideration on Mo's Algorithm
Mo's algorithm is an offline approach, which means we cannot use it when we are constrained to a specific order of queries or when update operations are involved.

When implementing Mo's algorithm, the most challenging aspect is implementing the functions `add` and `remove`. There are query types for which these operations are not as straightforward as in previous problems and require the use of more advanced data structures than just an array of counters. One of these cases is the range minimum queries (*RMQ*).

For RMQ, the addition and removal of an element needs maintaining the elements in the range within a Min-Heap, which increases the query time by a factor of $\log n$. Consequently, in this case, the amortized time per query is $\Theta(\sqrt{n}\log n)$, which is much worse than the ad hoc (and online) solution using a segment tree. This shouldn't come as a surprise, as ad hoc solutions that leverage specific properties of the problem at hand can often outperform general techniques like Mo's algorithm.

To conclude, let's consider an exercise that teaches the use of Mo's algorithm to solve queries on a tree.

*You have a rooted tree consisting of $$n$$ vertices. Each vertex of the tree has some color. We will assume that the tree vertices are numbered by integers from $$1$$ to $$n$$. Then we represent the color of vertex $$v$$ as $$c_v$$. The tree root is the vertex with number $$1$$.*

*We need to answer $$m$$ queries. Each query is described by two integers $$v_j,k_j$$. The answer to query $$v_j, k_j$$ is the number of colors $$c$$ that occur at least $$k_j$$ in the subtree of vertex $$v_j$$.*

This problem can be solved in $$\Theta((m+n)\sqrt{n})$$ time with the Mo's algorithm. How?

We should note that for this problem there exists a more advanced solution which runs in $$\Theta((n+q)\log n)$$ time. This solution uses to the [heavy-light decomposition](https://en.wikipedia.org/wiki/Heavy-light_decomposition) of the tree. How?
