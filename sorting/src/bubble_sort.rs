use std::fmt::Debug;

// Worst Time Complexity: O(N^2)
// Average Time Complexity: O(N^2)
// Space Complexity: O(1)

/// Performs bubble sort in place
pub fn bubble_sort<T: PartialOrd + Debug>(vec: &mut [T]) {
    let len = vec.len();

    for i in 0..len {
        let mut sorted = true;
        for j in 0..len - 1 - i {
            if vec[j] > vec[j+1] {
                sorted = false;
                vec.swap(j, j+1)
            }
        }

        if sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_sorted() {
        let mut vec_odd = [1,2,3,4,5,6,7];
        let mut vec_even = [0,1,2,3,4,5,6,7];

        bubble_sort(&mut vec_odd);
        bubble_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_unsorted() {
        let mut vec_odd = [4,7,3,6,1,5,2];
        let mut vec_even = [1,2,3,5,4,7,6,0];

        bubble_sort(&mut vec_odd);
        bubble_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }
}