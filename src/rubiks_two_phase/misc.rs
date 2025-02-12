// ######################################## Miscellaneous functions #####################################################

/// Rotate an array to the right between the indices `left` and `right` (inclusive).
pub fn rotate_right<T: Copy>(arr: &mut [T], left: usize, right: usize) {
    let temp = arr[right];
    for i in (left + 1..=right).rev() {
        arr[i] = arr[i - 1];
    }
    arr[left] = temp;
}

/// Rotate an array to the left between the indices `left` and `right` (inclusive).
pub fn rotate_left<T: Copy>(arr: &mut [T], left: usize, right: usize) {
    let temp = arr[left];
    for i in left..right {
        arr[i] = arr[i + 1];
    }
    arr[right] = temp;
}

/// Calculate the binomial coefficient "n choose k".
pub fn c_nk(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    let k = if k > n / 2 { n - k } else { k };
    let mut s = 1;
    let mut j = 1;
    for i in (n - k + 1)..=n {
        s *= i;
        s /= j;
        j += 1;
    }
    s
}

pub fn _factorial(x: usize) -> usize {
    (1..=x).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let mut arr = [1, 2, 3, 4, 5];
        rotate_right(&mut arr, 1, 3);
        assert_eq!(arr, [1, 4, 2, 3, 5]);
    }

    #[test]
    fn test_rotate_left() {
        let mut arr = [1, 2, 3, 4, 5];
        rotate_left(&mut arr, 1, 3);
        assert_eq!(arr, [1, 3, 4, 2, 5]);
    }

    #[test]
    fn test_c_nk() {
        assert_eq!(c_nk(5, 2), 10);
        assert_eq!(c_nk(6, 3), 20);
        assert_eq!(c_nk(0, 0), 1);
        assert_eq!(c_nk(6, 0), 1);
        assert_eq!(c_nk(6, 6), 1);
        assert_eq!(c_nk(6, 7), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(_factorial(2), 2);
        assert_eq!(_factorial(3), 6);
        assert_eq!(_factorial(4), 24);
        assert_eq!(_factorial(5), 120);
        assert_eq!(_factorial(6), 720);
        assert_eq!(_factorial(7), 5040);
    }
}
