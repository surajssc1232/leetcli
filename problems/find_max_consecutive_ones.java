// Find Max Consecutive Ones

Description:
// TODO: implement

class Solution {
    public int findMaxConsecutiveOnes(int[] nums) {
        // TODO: implement your solution here
    }
}

public class Main {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int testCount = 0;
        int passCount = 0;

        // Test Case 1
        testCount++;
        int[] nums1 = {1,1,0,1,1,1};
        int expected1 = 3;
        int actual1 = sol.findMaxConsecutiveOnes(nums1);
        if (actual1 == expected1) {
            System.out.println("Test Case 1: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 1: FAIL - Expected " + expected1 + ", Got " + actual1);
        }

        // Test Case 2
        testCount++;
        int[] nums2 = {1,0,1,1,0,1};
        int expected2 = 2;
        int actual2 = sol.findMaxConsecutiveOnes(nums2);
        if (actual2 == expected2) {
            System.out.println("Test Case 2: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 2: FAIL - Expected " + expected2 + ", Got " + actual2);
        }

        // Test Case 3
        testCount++;
        int[] nums3 = {0,0,0,0};
        int expected3 = 0;
        int actual3 = sol.findMaxConsecutiveOnes(nums3);
        if (actual3 == expected3) {
            System.out.println("Test Case 3: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 3: FAIL - Expected " + expected3 + ", Got " + actual3);
        }

        // Test Case 4: Single element 1
        testCount++;
        int[] nums4 = {1};
        int expected4 = 1;
        int actual4 = sol.findMaxConsecutiveOnes(nums4);
        if (actual4 == expected4) {
            System.out.println("Test Case 4: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 4: FAIL - Expected " + expected4 + ", Got " + actual4);
        }

        // Test Case 5: Empty array
        testCount++;
        int[] nums5 = {};
        int expected5 = 0;
        int actual5 = sol.findMaxConsecutiveOnes(nums5);
        if (actual5 == expected5) {
            System.out.println("Test Case 5: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 5: FAIL - Expected " + expected5 + ", Got " + actual5);
        }

        // Test Case 6: All ones
        testCount++;
        int[] nums6 = {1,1,1,1,1};
        int expected6 = 5;
        int actual6 = sol.findMaxConsecutiveOnes(nums6);
        if (actual6 == expected6) {
            System.out.println("Test Case 6: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 6: FAIL - Expected " + expected6 + ", Got " + actual6);
        }

        // Test Case 7: Alternating 0 and 1
        testCount++;
        int[] nums7 = {0,1,0,1,0,1,0};
        int expected7 = 1;
        int actual7 = sol.findMaxConsecutiveOnes(nums7);
        if (actual7 == expected7) {
            System.out.println("Test Case 7: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 7: FAIL - Expected " + expected7 + ", Got " + actual7);
        }

        // Test Case 8: Longer sequence of ones at the beginning
        testCount++;
        int[] nums8 = {1,1,1,1,0,0,1,1};
        int expected8 = 4;
        int actual8 = sol.findMaxConsecutiveOnes(nums8);
        if (actual8 == expected8) {
            System.out.println("Test Case 8: PASS");
            passCount++;
        } else {
            System.out.println("Test Case 8: FAIL - Expected " + expected8 + ", Got " + actual8);
        }


        System.out.println("\n--- Summary ---");
        System.out.println("Total Tests: " + testCount);
        System.out.println("Passed: " + passCount);
        System.out.println("Failed: " + (testCount - passCount));
    }
}
