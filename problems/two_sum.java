// Two Sum

class Solution {
    /**
     // TODO: implement
     *
     * You may assume that each input would have exactly one solution, and you may not use the same element twice.
     *
     // TODO: implement
     *
     * Example 1:
     * Input: nums = [2,7,11,15], target = 9
     * Output: [0,1]
     // TODO: implement
     *
     * Example 2:
     * Input: nums = [3,2,4], target = 6
     * Output: [1,2]
     *
     * Example 3:
     * Input: nums = [3,3], target = 6
     * Output: [0,1]
     *
     * Constraints:
     * 2 <= nums.length <= 10^4
     * -10^9 <= nums[i] <= 10^9
     * -10^9 <= target <= 10^9
     * Only one valid answer exists.
     */
    public int[] twoSum(int[] nums, int target) {
        // TODO: implement your solution here
        return new int[0]; // Placeholder
    }

    // Embedded Test Runner
    public static void main(String[] args) {
        Solution sol = new Solution();
        int testsPassed = 0;
        int totalTests = 0;

        // Test Case 1: Basic case
        totalTests++;
        int[] nums1 = {2, 7, 11, 15};
        int target1 = 9;
        int[] expected1 = {0, 1};
        int[] result1 = sol.twoSum(nums1, target1);
        if (areArraysEqual(result1, expected1) || areArraysEqual(result1, new int[]{expected1[1], expected1[0]})) {
            System.out.println("Test Case 1: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 1: FAIL");
        }

        // Test Case 2: Numbers not in order
        totalTests++;
        int[] nums2 = {3, 2, 4};
        int target2 = 6;
        int[] expected2 = {1, 2};
        int[] result2 = sol.twoSum(nums2, target2);
        if (areArraysEqual(result2, expected2) || areArraysEqual(result2, new int[]{expected2[1], expected2[0]})) {
            System.out.println("Test Case 2: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 2: FAIL");
        }

        // Test Case 3: Duplicate numbers
        totalTests++;
        int[] nums3 = {3, 3};
        int target3 = 6;
        int[] expected3 = {0, 1};
        int[] result3 = sol.twoSum(nums3, target3);
        if (areArraysEqual(result3, expected3) || areArraysEqual(result3, new int[]{expected3[1], expected3[0]})) {
            System.out.println("Test Case 3: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 3: FAIL");
        }

        // Test Case 4: Negative numbers
        totalTests++;
        int[] nums4 = {-1, -3, 5, 8};
        int target4 = 4;
        int[] expected4 = {0, 2};
        int[] result4 = sol.twoSum(nums4, target4);
        if (areArraysEqual(result4, expected4) || areArraysEqual(result4, new int[]{expected4[1], expected4[0]})) {
            System.out.println("Test Case 4: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 4: FAIL");
        }

        // Test Case 5: Large numbers
        totalTests++;
        int[] nums5 = {1000000000, 5, 1000000000, 15};
        int target5 = 1000000015;
        int[] expected5 = {0, 3};
        int[] result5 = sol.twoSum(nums5, target5);
        if (areArraysEqual(result5, expected5) || areArraysEqual(result5, new int[]{expected5[1], expected5[0]})) {
            System.out.println("Test Case 5: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 5: FAIL");
        }
        
        // Test Case 6: Target is zero, need a negative and a positive
        totalTests++;
        int[] nums6 = {-5, 0, 5, 10};
        int target6 = 0;
        int[] expected6 = {0, 2};
        int[] result6 = sol.twoSum(nums6, target6);
        if (areArraysEqual(result6, expected6) || areArraysEqual(result6, new int[]{expected6[1], expected6[0]})) {
            System.out.println("Test Case 6: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 6: FAIL");
        }

        // Test Case 7: Target requires the two largest numbers
        totalTests++;
        int[] nums7 = {1, 2, 3, 4, 5};
        int target7 = 9;
        int[] expected7 = {3, 4};
        int[] result7 = sol.twoSum(nums7, target7);
        if (areArraysEqual(result7, expected7) || areArraysEqual(result7, new int[]{expected7[1], expected7[0]})) {
            System.out.println("Test Case 7: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 7: FAIL");
        }

        System.out.println("\n--- Test Summary ---");
        System.out.println("Tests Passed: " + testsPassed + "/" + totalTests);
    }

    // Helper function to compare arrays, considering order doesn't matter
    private static boolean areArraysEqual(int[] arr1, int[] arr2) {
        if (arr1 == null || arr2 == null || arr1.length != arr2.length) {
            // TODO: implement
        }
        if (arr1.length == 0 && arr2.length == 0) {
            // TODO: implement
        }
        // Sort both arrays to compare elements regardless of order
        java.util.Arrays.sort(arr1);
        java.util.Arrays.sort(arr2);
        // TODO: implement
    }
}
