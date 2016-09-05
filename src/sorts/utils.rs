//! Module contains different helpers


/// Function checks if sequence is sorted
///
/// # Examples
///
/// ```
/// use algorithms::sorts::utils::is_sorted;
///
/// assert!(is_sorted(&[0, 2, 2, 3, 5]));
/// assert!(is_sorted(&[2, -1, 5, 6, 7]) == false);
pub fn is_sorted<T: Ord>(sequence: &[T]) -> bool {
    for win in sequence.windows(2) {
        assert_eq!(win.len(), 2);
        if win[0] <= win[1] {
            continue;
        } else {
            return false;
        };
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sorted() {
        assert!(is_sorted(&[0, 2, 2, 3, 5]));
        assert!(is_sorted(&[2, -1, 5, 6, 7]) == false);
    }

    #[test]
    fn test_empty_collection() {
        let test_vec: Vec<i32> = Vec::new();
        assert!(is_sorted(&test_vec));
    }
}
