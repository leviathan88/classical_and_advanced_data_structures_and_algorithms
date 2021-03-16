use std::fmt::Debug;

// Worst Time Complexity: O(N * log(N))
// Average Time Complexity: O(N * log(N))
// Space Complexity: O(N)

/// Performs merge sort by consuming the vec![] and returning newly sorted vec![]
pub fn merge_sort<T: PartialOrd + Debug>(mut vec: Vec<T>) -> Vec<T> {
    let len = vec.len();

    if len <= 1 {
        return vec;
    } else {
        let right = vec.split_off(len / 2);
        let right_sorted = merge_sort(right);
        let left_sorted = merge_sort(vec);

        let mut right_iter = right_sorted.into_iter();
        let mut left_iter = left_sorted.into_iter();

        let mut right_peek = right_iter.next();
        let mut left_peek = left_iter.next();

        let mut result = Vec::with_capacity(len);

        loop {
            match right_peek {
                Some(ref right_value) => match left_peek {
                    Some(ref left_value) => {
                        if right_value >= left_value {
                            result.push(left_peek.take().unwrap());
                            left_peek = left_iter.next();
                        } else {
                            result.push(right_peek.take().unwrap());
                            right_peek = right_iter.next();
                        }
                    },
                    None => {
                        result.push(right_peek.take().unwrap());
                        result.extend(right_iter);
                        break;    
                    }
                },
                None => {
                    if let Some(left_value) = left_peek {
                        result.push(left_value);
                    }
                    result.extend(left_iter);
                    break;
                }
            }
        }

        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_sorted() {
        let vec_odd = vec![1,2,3,4,5,6,7];
        let vec_even = vec![0,1,2,3,4,5,6,7];

        assert_eq!(merge_sort(vec_odd), [1,2,3,4,5,6,7]);
        assert_eq!(merge_sort(vec_even), [0,1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_unsorted() {
        let vec_odd = vec![4,7,3,6,1,5,2];
        let vec_even = vec![1,2,3,5,4,7,6,0];
        
        assert_eq!(merge_sort(vec_odd), [1,2,3,4,5,6,7]);
        assert_eq!(merge_sort(vec_even), [0,1,2,3,4,5,6,7]);
    }
}