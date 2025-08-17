# Find Maximum Consecutive Ones
# Given a binary array nums, return the maximum number of consecutive 1's in the array.
#
# Example 1:
# Input: nums = [1,1,0,1,1,1]
# Output: 3
# Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
#
# Example 2:
# Input: nums = [1,0,1,1,0,1]
# Output: 2
# Explanation: The maximum number of consecutive 1s is 2.

def find_max_consecutive_ones(nums):
    # TODO: implement your solution here
    pass

def run_tests():
    test_cases = [
        ([1,1,0,1,1,1], 3),
        ([1,0,1,1,0,1], 2),
        ([0,0,0,0], 0),
        ([1,1,1,1,1], 5),
        ([], 0),
        ([1], 1),
        ([0], 0),
        ([0,1,1,0,1,1,1,0,1], 3),
        ([1,0,0,0,1,1,0,1,1,1,1], 4)
    ]
    
    passed = 0
    for i, (nums, expected) in enumerate(test_cases):
        try:
            result = find_max_consecutive_ones(nums)
            if result == expected:
                print(f"✓ Test {i+1} PASSED")
                passed += 1
            else:
                print(f"✗ Test {i+1} FAILED: expected {expected}, got {result}")
        except Exception as e:
            print(f"✗ Test {i+1} ERROR: {e}")
    
    print(f"\nResult: {passed}/{len(test_cases)} tests passed")

if __name__ == "__main__":
    run_tests()
