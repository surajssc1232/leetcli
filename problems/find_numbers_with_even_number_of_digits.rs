// Find Numbers with Even Number of Digits
// Given an array nums of integers, return how many of them contain an even number of digits.
//
// Example 1:
// Input: nums = [12,345,2,6,7896]
// Output: 2
// Explanation:
// 12 contains 2 digits (even number of digits).
// 345 contains 3 digits (odd number of digits).
// 2 contains 1 digit (odd number of digits).
// 6 contains 1 digit (odd number of digits).
// 7896 contains 4 digits (even number of digits).
//
// Example 2:
// Input: nums = [555,901,482,1771]
// Output: 1
// Explanation:
// 555 contains 3 digits (odd number of digits).
// 901 contains 3 digits (odd number of digits).
// 482 contains 3 digits (odd number of digits).
// 1771 contains 4 digits (even number of digits).

fn find_numbers(nums: Vec<i32>) -> i32 {
    // TODO: implement
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_numbers() {
        assert_eq!(find_numbers(vec![12,345,2,6,7896]), 2);
        assert_eq!(find_numbers(vec![555,901,482,1771]), 1);
        assert_eq!(find_numbers(vec![1, 22, 333, 4444, 55555]), 2);
        assert_eq!(find_numbers(vec![10, 100, 1000, 10000]), 2);
        assert_eq!(find_numbers(vec![1, 3, 5, 7, 9]), 0);
        assert_eq!(find_numbers(vec![10, 1000, 100000]), 1);
    }
}