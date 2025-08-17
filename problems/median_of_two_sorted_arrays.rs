// Median of Two Sorted Arrays
// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// The overall run time complexity should be O(log (m+n)).
//
// Example 1:
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
//
// Example 2:
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
// Example 3:
// Input: nums1 = [0,0], nums2 = [0,0]
// Output: 0.00000
//
// Example 4:
// Input: nums1 = [], nums2 = [1]
// Output: 1.00000
//
// Example 5:
// Input: nums1 = [2], nums2 = []
// Output: 2.00000

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // TODO: implement
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.00000);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.50000);
        assert_eq!(find_median_sorted_arrays(vec![0,0], vec![0,0]), 0.00000);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.00000);
        assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.00000);
        assert_eq!(find_median_sorted_arrays(vec![1, 5, 8, 10], vec![2, 3, 6, 9, 11, 12]), 7.00000);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 5.50000);
    }
}