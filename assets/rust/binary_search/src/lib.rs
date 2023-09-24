fn binary_search<T: Ord>(arr: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / 2;

        match key.cmp(&arr[middle]) {
            std::cmp::Ordering::Equal => {
                ans = Some(middle);
                high = middle
            }
            std::cmp::Ordering::Less => high = middle,
            std::cmp::Ordering::Greater => low = middle + 1,
        }
    }

    ans
}

fn exponential_search<T: Ord + std::fmt::Debug + Copy>(arr: &[T], key: T) -> Option<usize> {
    let mut high = 1;
    while high < arr.len() {
        match key.cmp(&arr[high]) {
            std::cmp::Ordering::Equal => {
                high += 1;
                break;
            }
            std::cmp::Ordering::Less => break,
            std::cmp::Ordering::Greater => high *= 2,
        }
    }

    let low = high / 2;
    high = high.min(arr.len());

    let r = binary_search(&arr[low..high], key)? + low;

    Some(r)
}

use num::FromPrimitive;
use num::Num;
use std::cmp::PartialOrd;

fn binary_search_range<T, F>(low: T, high: T, pred: F) -> Option<T>
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    let mut low = low;
    let mut high = high;

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / FromPrimitive::from_u64(2).unwrap();

        match pred(middle) {
            true => {
                low = middle + T::one();
                ans = Some(middle)
            }
            false => high = middle,
        }
    }

    ans
}

fn sqrt(v: u64) -> u64 {
    binary_search_range(0, v + 1, |x| x * x <= v).unwrap()
}

fn select_intervals(intervals: &mut Vec<(usize, usize)>, c: usize) -> Option<usize> {
    intervals.sort_unstable();

    let l = intervals
        .iter()
        .fold(0, |acc, interval| acc + interval.1 - interval.0 + 1); // overall length

    if l < c {
        // there is no solution
        return None;
    }

    // A closure implements our predicate
    let pred = |d: usize| -> bool {
        let mut last_selected = intervals[0].0;
        let mut cnt = 1;
        for &interval in intervals.iter() {
            while interval.0.max(last_selected + d) <= interval.1 {
                last_selected = interval.0.max(last_selected + d);
                cnt += 1;
            }
        }

        cnt >= c
    };

    binary_search_range(1, l + 1, pred)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_intervals() {
        assert_eq!(select_intervals(&mut vec![(0, 0), (1, 4)], 6), None);
        assert_eq!(select_intervals(&mut vec![(0, 0), (1, 5)], 6), Some(1));
        assert_eq!(
            select_intervals(&mut vec![(0, 2), (4, 7), (9, 9)], 5),
            Some(2)
        );
        assert_eq!(
            select_intervals(&mut vec![(0, 2), (4, 7), (9, 9)], 5),
            Some(2)
        );
    }

    #[test]
    fn test_binary_search() {
        let v = vec![0, 1, 2, 3, 4, 6, 7, 8, 9];
        assert_eq!(binary_search(&v[..], 3), Some(3));
        assert_eq!(binary_search(&v[..], 10), None);
        assert_eq!(binary_search(&v[..], -1), None);
        assert_eq!(binary_search(&v[..], 5), None);

        let v = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        assert_eq!(binary_search(&v[..], 1), Some(0));
        assert_eq!(binary_search(&v[..], 2), Some(3));
        assert_eq!(binary_search(&v[..], 3), Some(6));

        let v = vec!["aaa", "abc", "bca"];
        assert_eq!(binary_search(&v[..], "abc"), Some(1));
        assert_eq!(binary_search(&v[..], "a"), None);
        assert_eq!(binary_search(&v[..], "c"), None);
    }

    #[test]
    fn test_exponential_search() {
        let v = vec![0, 1, 2, 3, 4, 6, 7, 8, 9];
        assert_eq!(exponential_search(&v[..], 3), Some(3));
        assert_eq!(exponential_search(&v[..], 10), None);
        assert_eq!(exponential_search(&v[..], -1), None);
        assert_eq!(exponential_search(&v[..], 5), None);

        let v = vec!["aaa", "abc", "bca"];
        assert_eq!(exponential_search(&v[..], "abc"), Some(1));
        assert_eq!(exponential_search(&v[..], "a"), None);
        assert_eq!(exponential_search(&v[..], "c"), None);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(0), 0);
        assert_eq!(sqrt(1), 1);
        assert_eq!(sqrt(2), 1);
        assert_eq!(sqrt(4), 2);
        assert_eq!(sqrt(8), 2);
        assert_eq!(sqrt(9), 3);
        assert_eq!(sqrt(24), 4);
        assert_eq!(sqrt(25), 5);
        assert_eq!(sqrt(26), 5);
    }
}
