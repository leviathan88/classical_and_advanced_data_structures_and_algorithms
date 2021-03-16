use std::fmt::Debug;

fn pivot<T: PartialOrd>(vec: &mut [T]) -> usize {
    let len = vec.len();
    let mut p = 0;

    for i in 1..len {
        if vec[i] < vec[p] {
            vec.swap(p + 1, i);
            vec.swap(p, p + 1);
            p += 1;
        }
    }

    p
}

// Worst Time Complexity: O(N^2)
// Average Time Complexity: O(N * log(N))
// Space Complexity: O(N)

/// Performs merge sort by consuming the vec![] and returning newly sorted vec![]
pub fn quick_sort<T: PartialOrd + Debug>(vec: &mut [T]) {
    let len = vec.len();

    if len <= 1 {
        return;
    }

    let p = pivot(vec);
    
    let (a, b) = vec.split_at_mut(p);

    quick_sort(a);
    quick_sort(&mut b[1..]);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test_sorted() {
        let mut vec_odd = [1,2,3,4,5,6,7];
        let mut vec_even = [0,1,2,3,4,5,6,7];

        quick_sort(&mut vec_odd);
        quick_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_unsorted() {
        let mut vec_odd = [4,7,3,6,1,5,2];
        let mut vec_even = [1,2,3,5,4,7,6,0];

        quick_sort(&mut vec_odd);
        quick_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }
}