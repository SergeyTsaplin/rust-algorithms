//! Module contains pure implementations of insertion sort algorithm.
//!
//! * `sort` - is non distructive function (does not affects on input sequence)
//! * `mut_sort` - function with side effects (modify input sequence)
//!
//! It's better to use first one in functional style, second - in OOP-style

/// Pure implementation of insertion sort algorithm. Side effect free
///
/// # Examples
///
/// ```
/// use algorithms::sorts::insertion_sort::sort;
/// use algorithms::sorts::utils::is_sorted;
///
/// assert!(is_sorted(&sort(&[0, 5, 3, 2, 2])));
/// assert!(is_sorted(&sort(&[-1, -5, -10, 105])));
/// ```
pub fn sort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    let mut sorted_list = list.clone().to_vec();
    let length = sorted_list.len();
    if length <= 1 {
        return sorted_list;
    }
    for i in 1..length {
        let current_item = sorted_list.remove(i);
        let mut j = i;
        let mut has_greater = false;
        while j >= 1 && current_item < sorted_list[j - 1] {
            has_greater = true;
            j -= 1;
        }
        let index = if has_greater {
            j
        } else {
            i
        };
        sorted_list.insert(index, current_item)
    }
    return sorted_list;
}

/// Pure implementation of bubble sort algorithm. Side-effects version
/// Modifies input sequense
///
/// # Examples
///
/// ```
/// use algorithms::sorts::bubble_sort::mut_sort;
/// use algorithms::sorts::utils::is_sorted;
///
/// let mut test_sequence = [0, 5, 3, 2, 2];
/// mut_sort(&mut test_sequence);
/// assert!(is_sorted(&test_sequence));
///
/// let mut test_sequence = [0, 5, 3, 2, 2];
/// mut_sort(&mut test_sequence);
/// assert!(is_sorted(&test_sequence));
/// ```
pub fn mut_sort<T: Ord>(list: &mut Vec<T>) {
    let length = list.len();
    if length <= 1 {
        return;
    }
    for i in 1..length {
        let current_item = list.remove(i);
        let mut j = i;
        let mut has_greater = false;
        while j >= 1 && current_item < list[j - 1] {
            has_greater = true;
            j -= 1;
        }
        let index = if has_greater {
            j
        } else {
            i
        };
        list.insert(index, current_item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils::is_sorted;

    #[test]
    fn sort_numbers() {
        let empty_vec: Vec<i32> = Vec::new();
        assert!(is_sorted(&sort(&[0, 5, 3, 2, 2])));
        assert!(is_sorted(&sort(&empty_vec)));
        assert!(is_sorted(&sort(&[-1, -5, -10, 105])));
    }

    #[test]
    fn sort_chars() {
        assert!(is_sorted(&sort(&['a', 'r', 'b'])))
    }

    #[test]
    fn mut_sort_numbers() {
        let mut test_vector = vec![0, 5, 2, 3, 2];
        mut_sort(&mut test_vector);
        assert!(is_sorted(&test_vector));
        let mut test_vector: Vec<i32> = Vec::new();
        mut_sort(&mut test_vector);
        assert!(is_sorted(&test_vector));
    }

    #[test]
    fn mut_sort_chars() {
        let mut test_vector = vec!['a', 'r', 'b'];
        mut_sort(&mut test_vector);
        assert!(is_sorted(&test_vector));
    }
}
