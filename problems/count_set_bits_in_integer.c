// Count Set Bits in Integer

// TODO: implement

**Examples:**

1.  **Input:** `n = 2`
    **Binary Representation:** `10`
    **Output:** `1` (There is one '1' in the binary representation of 2.)

2.  **Input:** `n = 7`
    **Binary Representation:** `111`
    **Output:** `3` (There are three '1's in the binary representation of 7.)

3.  **Input:** `n = 0`
    **Binary Representation:** `0`
    **Output:** `0` (There are no '1's in the binary representation of 0.)

4.  **Input:** `n = 15`
    **Binary Representation:** `1111`
    **Output:** `4`

int countSetBits(int n) {
    // TODO: implement your solution here
}

#include <stdio.h>
#include <stdbool.h>

// Helper function to run a single test case
bool runTest(int input, int expected_output, const char* test_name) {
    int actual_output = countSetBits(input);
    if (actual_output == expected_output) {
        printf("%s: PASS\n", test_name);
        // TODO: implement
    } else {
        printf("%s: FAIL (Input: %d, Expected: %d, Got: %d)\n", test_name, input, expected_output, actual_output);
        // TODO: implement
    }
}

int main() {
    int passed_tests = 0;
    int total_tests = 0;

    // Test Case 1: Basic case
    total_tests++;
    if (runTest(2, 1, "Test Case 1 (Basic)")) passed_tests++;

    // Test Case 2: Multiple set bits
    total_tests++;
    if (runTest(7, 3, "Test Case 2 (Multiple Bits)")) passed_tests++;

    // Test Case 3: Zero
    total_tests++;
    if (runTest(0, 0, "Test Case 3 (Zero)")) passed_tests++;

    // Test Case 4: All bits set (for a small number)
    total_tests++;
    if (runTest(15, 4, "Test Case 4 (All Bits Set)")) passed_tests++;

    // Test Case 5: A larger number with mixed bits
    total_tests++;
    if (runTest(42, 3, "Test Case 5 (Larger Number)")) passed_tests++; // Binary of 42 is 101010

    // Test Case 6: Power of 2
    total_tests++;
    if (runTest(16, 1, "Test Case 6 (Power of 2)")) passed_tests++; // Binary of 16 is 10000

    // Test Case 7: Another larger number
    total_tests++;
    if (runTest(100, 3, "Test Case 7 (Another Large)")) passed_tests++; // Binary of 100 is 1100100

    // Test Case 8: Maximum possible integer value (demonstrates a large input)
    // For a 32-bit integer, this would be INT_MAX. The number of set bits depends on the exact value.
    // Let's use a known large value.
    total_tests++;
    if (runTest(2147483647, 31, "Test Case 8 (Max Int - 1)")) passed_tests++; // INT_MAX is 2^31 - 1

    printf("\n--- Test Summary ---\n");
    printf("Passed: %d/%d\n", passed_tests, total_tests);
    if (passed_tests == total_tests) {
        printf("All tests passed!\n");
    } else {
        printf("Some tests failed.\n");
    }

    // TODO: implement
}