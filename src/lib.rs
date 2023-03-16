pub fn bubble_sort(n: &mut [i32]) {
    let len = n.len();

    for i in 0..len {
        for j in 0..len-i-1 {
            if n[j] > n[j+1] {
                n.swap(j, j+1);
            }
        }
    }
}

pub fn bubble_sort_generic<T: PartialOrd>(n: &mut [T]) {
    let len = n.len();

    for i in 0..len {
        for j in 0..len-i-1 {
            if n[j] > n[j+1] {
                n.swap(j, j+1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums = [8, 7, 5, 6, 9];
        bubble_sort(&mut nums);
        assert_eq!(nums, [5, 6, 7, 8, 9]);
    }
    
    #[test]
    fn test_bubble_sort_generic() {
        let mut nums = [4.4, 2.2, 3.3, 1.1, 5.5];
        bubble_sort_generic(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);

        let mut strings = ["c", "a", "d", "b"];
        bubble_sort_generic(&mut strings);
        assert_eq!(strings, ["a", "b", "c", "d"]);
    }
}
