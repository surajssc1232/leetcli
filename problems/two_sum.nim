// Two Sum
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// You can return the answer in any order.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
// Constraints:
//
// 2 <= nums.length <= 10^4
// -10^9 <= nums[i] <= 10^9
// -10^9 <= target <= 10^9
// Only one valid answer exists.

import std/tables
import std/sequtils
import std/strutils

proc twoSum(nums: seq[int], target: int): seq[int] =
  # TODO: implement your solution here

proc runTest(testName: string, nums: seq[int], target: int, expected: seq[int]) =
  let result = twoSum(nums, target)
  var sortedResult = result.sorted()
  var sortedExpected = expected.sorted()

  if sortedResult == sortedExpected:
    echo testName & ": PASS"
  else:
    echo testName & ": FAIL - Input nums: " & $nums & ", target: " & $target & ", Expected: " & $sortedExpected & ", Got: " & $sortedResult

proc main() =
  runTest("Example 1", @[2, 7, 11, 15], 9, @[0, 1])
  runTest("Example 2", @[3, 2, 4], 6, @[1, 2])
  runTest("Example 3", @[3, 3], 6, @[0, 1])
  runTest("Negative Numbers", @[-1, -3, -5, -7], -8, @[1, 3])
  runTest("Mixed Numbers", @[-2, 7, 11, -15], 5, @[0, 1])
  runTest("Larger Array", @[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 17, @[6, 8])
  runTest("Target is Zero", @[-1, 0, 1], 0, @[0, 2])
  runTest("Large Target", @[1000000000, 2, 3], 1000000002, @[0, 1])

when isMainModule:
  main()
