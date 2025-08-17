# Two Sum Test Problem
# Find two numbers in an array that add up to target

def two_sum(nums, target):
    # Correct implementation
    num_to_index = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_to_index:
            return [num_to_index[complement], i]
        num_to_index[num] = i
    return []

def run_tests():
    test_cases = [
        ([2,7,11,15], 9, [0,1]),
        ([3,2,4], 6, [1,2]),
        ([3,3], 6, [0,1]),
    ]
    
    passed = 0
    for i, (nums, target, expected) in enumerate(test_cases):
        try:
            result = two_sum(nums, target)
            if result and sorted(result) == sorted(expected):
                print(f"✓ Test {i+1} PASSED")
                passed += 1
            else:
                print(f"✗ Test {i+1} FAILED: expected {expected}, got {result}")
        except Exception as e:
            print(f"✗ Test {i+1} ERROR: {e}")
    
    print(f"\nResult: {passed}/{len(test_cases)} tests passed")
    return passed == len(test_cases)

if __name__ == "__main__":
    run_tests()