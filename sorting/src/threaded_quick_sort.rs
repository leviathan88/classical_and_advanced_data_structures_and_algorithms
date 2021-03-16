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

// in order to make this work with Threads
// we needed a private struct that would implement Send
// which holds our raw mutable pointer

// also we needed to add 'static, cuz it only works for static types

struct RawSend<T>(*mut [T]); // single element tuple
unsafe impl<T> Send for RawSend<T> {}

// Worst Time Complexity: O(N^2)
// Average Time Complexity: O(N * log(N))
// Space Complexity: O(N)

/// Performs merge sort by consuming the vec![] and returning newly sorted vec![]
pub fn threaded_quick_sort<T:'static + PartialOrd + Debug + Send>(vec: &mut [T]) {
    let len = vec.len();

    if len <= 1 {
        return;
    }

    let p = pivot(vec);
    
    let (a, b) = vec.split_at_mut(p);

    // it was a mutable borrow, now it's a mutable raw pointer
    let raw_a: *mut [T] = a as *mut [T];
    let raw_send_a = RawSend(raw_a);

    // SAFETY: this thread returns before the function finishes
    // that is guaranteed by calling handle.join() below
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_send_a.0);
        });

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();

    }
}

#[cfg(test)]
mod tests {
    use super::threaded_quick_sort;

    #[test]
    fn test_sorted() {
        let mut vec_odd = [1,2,3,4,5,6,7];
        let mut vec_even = [0,1,2,3,4,5,6,7];

        threaded_quick_sort(&mut vec_odd);
        threaded_quick_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_unsorted() {
        let mut vec_odd = [4,7,3,6,1,5,2];
        let mut vec_even = [1,2,3,5,4,7,6,0];

        threaded_quick_sort(&mut vec_odd);
        threaded_quick_sort(&mut vec_even);
        
        assert_eq!(vec_odd, [1,2,3,4,5,6,7]);
        assert_eq!(vec_even, [0, 1,2,3,4,5,6,7]);
    }
}