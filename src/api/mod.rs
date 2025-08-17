use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub async fn generate_problem_with_tests(topic: &str, language: &str, api_key: &str, model: &str, difficulty: &str) -> Result<String> {
    let client = Client::new();
    
    let prompt = format!(
        "CRITICAL: The very first line must be the problem title as a comment.
Comment formats: // (C, C++, Java, JavaScript, Rust, Zig, Go), # (Python, Nim), (* *) (Pascal), ; (Assembly)

Generate a LeetCode-style {} problem in {} with {} difficulty that includes EMBEDDED TEST CASES.

DIFFICULTY GUIDELINES:
- Easy: Simple logic, basic data structures, straightforward algorithms
- Medium: Moderate complexity, multiple steps, common algorithms like binary search, DFS/BFS
- Hard: Complex algorithms, advanced data structures, optimization problems

EXACT FORMAT REQUIRED:
1. Problem title as the very first comment line using proper comment syntax for the language
2. Problem description and examples AS COMMENTS using the language's comment syntax
3. Function signature with empty body containing 'TODO: implement your solution here'
4. EMBEDDED TEST RUNNER with at least 5 diverse test cases
5. Test runner that prints PASS/FAIL for each test and final summary
6. NO solution logic in the main function - just the skeleton

Example structure for {}:
{}

CRITICAL REQUIREMENTS:
- Function body must be EMPTY (just TODO comment)
- Problem description and examples MUST be in comments using proper syntax for {}
- Test runner must work when the function is properly implemented
- Test cases should cover edge cases (empty inputs, single elements, large inputs)
- Print clear PASS/FAIL messages for each test
- Include a main section that runs all tests
- NO markdown formatting or code blocks in response

Generate a random {} {} difficulty problem following this EXACT format with comprehensive test coverage.",
        topic,
        language,
        difficulty,
        language,
        get_enhanced_skeleton_example(language),
        language,
        topic,
        difficulty
    );
    
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, api_key);
    
    let request_body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.9,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 2048
        }
    });

    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("API request failed: {}", error_text));
    }

    let response_json: serde_json::Value = response.json().await?;
    
    let content = response_json
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(|t| t.as_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse API response. Response: {}", serde_json::to_string_pretty(&response_json).unwrap_or_default()))?;

    let cleaned_content = clean_content(content);
    Ok(cleaned_content)
}

fn clean_content(content: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("```") {
            continue;
        }
        
        if trimmed.contains("return ") && 
           !trimmed.contains("TODO") && 
           !trimmed.starts_with("//") && 
           !trimmed.starts_with("#") &&
           !trimmed.contains("return new int[0]") && 
           !trimmed.contains("return {}") &&        
           !trimmed.contains("return [];") &&       
           !trimmed.contains("return []int{}") {   
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            lines.push(format!("{}// TODO: implement", indent));
            continue;
        }
        
        lines.push(line.to_string());
    }
    
    lines.join("\n")
}

fn get_enhanced_skeleton_example(language: &str) -> &'static str {
    match language {
        "Python" => {
            r#"# Two Sum
# Given an array of integers nums and an integer target,
# return indices of two numbers that add up to target.
#
# Example 1:
# Input: nums = [2,7,11,15], target = 9
# Output: [0,1]
# Explanation: nums[0] + nums[1] = 2 + 7 = 9
#
# Example 2:
# Input: nums = [3,2,4], target = 6
# Output: [1,2]

def two_sum(nums, target):
    # TODO: implement your solution here
    pass

def run_tests():
    test_cases = [
        ([2,7,11,15], 9, [0,1]),
        ([3,2,4], 6, [1,2]),
        ([3,3], 6, [0,1]),
        ([1,2,3,4], 7, [2,3]),
        ([], 0, [])
    ]
    
    passed = 0
    for i, (nums, target, expected) in enumerate(test_cases):
        try:
            result = two_sum(nums, target)
            if sorted(result) == sorted(expected):
                print(f"✓ Test {i+1} PASSED")
                passed += 1
            else:
                print(f"✗ Test {i+1} FAILED: expected {expected}, got {result}")
        except Exception as e:
            print(f"✗ Test {i+1} ERROR: {e}")
    
    print(f"\nResult: {passed}/{len(test_cases)} tests passed")
    return passed == len(test_cases)

if __name__ == "__main__":
    run_tests()"#
        },
        "Rust" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // TODO: implement your solution here
    vec![]
}

fn run_tests() -> bool {
    let test_cases = vec![
        (vec![2,7,11,15], 9, vec![0,1]),
        (vec![3,2,4], 6, vec![1,2]),
        (vec![3,3], 6, vec![0,1])
    ];
    
    let mut passed = 0;
    for (i, (nums, target, expected)) in test_cases.iter().enumerate() {
        let result = two_sum(nums.clone(), *target);
        if result == *expected || (result.len() == expected.len() && 
           result.iter().all(|x| expected.contains(x))) {
            println!("✓ Test {} PASSED", i + 1);
            passed += 1;
        } else {
            println!("✗ Test {} FAILED: expected {:?}, got {:?}", i + 1, expected, result);
        }
    }
    
    println!("\nResult: {}/{} tests passed", passed, test_cases.len());
    passed == test_cases.len()
}

fn main() {
    run_tests();
}"#
        },
        "JavaScript" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

function twoSum(nums, target) {
    // TODO: implement your solution here
    return [];
}

function runTests() {
    const testCases = [
        [[2,7,11,15], 9, [0,1]],
        [[3,2,4], 6, [1,2]],
        [[3,3], 6, [0,1]]
    ];
    
    let passed = 0;
    testCases.forEach((testCase, i) => {
        const [nums, target, expected] = testCase;
        try {
            const result = twoSum(nums, target);
            if (JSON.stringify(result.sort()) === JSON.stringify(expected.sort())) {
                console.log(`✓ Test ${i+1} PASSED`);
                passed++;
            } else {
                console.log(`✗ Test ${i+1} FAILED: expected ${JSON.stringify(expected)}, got ${JSON.stringify(result)}`);
            }
        } catch (e) {
            console.log(`✗ Test ${i+1} ERROR: ${e.message}`);
        }
    });
    
    console.log(`\nResult: ${passed}/${testCases.length} tests passed`);
    return passed === testCases.length;
}

runTests();"#
        },
        "Java" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

class Solution {
    public int[] twoSum(int[] nums, int target) {
        // TODO: implement your solution here
        return new int[0];
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        int testsPassed = 0;
        int totalTests = 0;

        // Test Case 1
        totalTests++;
        int[] nums1 = {2, 7, 11, 15};
        int target1 = 9;
        int[] expected1 = {0, 1};
        int[] result1 = sol.twoSum(nums1, target1);
        if (areArraysEqual(result1, expected1)) {
            System.out.println("Test Case 1: PASS");
            testsPassed++;
        } else {
            System.out.println("Test Case 1: FAIL");
        }

        System.out.println("\n--- Test Summary ---");
        System.out.println("Tests Passed: " + testsPassed + "/" + totalTests);
    }

    private static boolean areArraysEqual(int[] arr1, int[] arr2) {
        if (arr1.length != arr2.length) return false;
        java.util.Arrays.sort(arr1);
        java.util.Arrays.sort(arr2);
        return java.util.Arrays.equals(arr1, arr2);
    }
}"#
        },
        "C" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    // TODO: implement your solution here
    *returnSize = 0;
    return NULL;
}

bool runTest(int* nums, int numsSize, int target, int* expected, int expectedSize, const char* testName) {
    int returnSize;
    int* result = twoSum(nums, numsSize, target, &returnSize);
    
    if (returnSize == expectedSize) {
        // Check if arrays match (considering order)
        bool match = true;
        for (int i = 0; i < returnSize; i++) {
            if (result[i] != expected[i]) {
                match = false;
                break;
            }
        }
        if (match) {
            printf("%s: PASS\n", testName);
            free(result);
            return true;
        }
    }
    
    printf("%s: FAIL\n", testName);
    if (result) free(result);
    return false;
}

int main() {
    int passed = 0;
    int total = 0;
    
    // Test Case 1
    total++;
    int nums1[] = {2, 7, 11, 15};
    int expected1[] = {0, 1};
    if (runTest(nums1, 4, 9, expected1, 2, "Test Case 1")) passed++;
    
    printf("\n--- Test Summary ---\n");
    printf("Passed: %d/%d\n", passed, total);
    
    return 0;
}"#
        },
        "C++" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

#include <vector>
#include <iostream>
#include <algorithm>

class Solution {
public:
    std::vector<int> twoSum(std::vector<int>& nums, int target) {
        // TODO: implement your solution here
        return {};
    }
};

bool runTest(std::vector<int> nums, int target, std::vector<int> expected, const std::string& testName) {
    Solution sol;
    std::vector<int> result = sol.twoSum(nums, target);
    
    std::sort(result.begin(), result.end());
    std::sort(expected.begin(), expected.end());
    
    if (result == expected) {
        std::cout << testName << ": PASS" << std::endl;
        return true;
    } else {
        std::cout << testName << ": FAIL" << std::endl;
        return false;
    }
}

int main() {
    int passed = 0, total = 0;
    
    // Test Case 1
    total++;
    if (runTest({2, 7, 11, 15}, 9, {0, 1}, "Test Case 1")) passed++;
    
    std::cout << "\n--- Test Summary ---" << std::endl;
    std::cout << "Passed: " << passed << "/" << total << std::endl;
    
    return 0;
}"#
        },
        "Go" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

package main

import (
    "fmt"
    "sort"
)

func twoSum(nums []int, target int) []int {
    // TODO: implement your solution here
    return []int{}
}

func runTest(nums []int, target int, expected []int, testName string) bool {
    result := twoSum(nums, target)
    
    // Sort both for comparison
    sortedResult := make([]int, len(result))
    copy(sortedResult, result)
    sort.Ints(sortedResult)
    
    sortedExpected := make([]int, len(expected))
    copy(sortedExpected, expected)
    sort.Ints(sortedExpected)
    
    if len(sortedResult) == len(sortedExpected) {
        match := true
        for i := range sortedResult {
            if sortedResult[i] != sortedExpected[i] {
                match = false
                break
            }
        }
        if match {
            fmt.Printf("%s: PASS\n", testName)
            return true
        }
    }
    
    fmt.Printf("%s: FAIL\n", testName)
    return false
}

func main() {
    passed, total := 0, 0
    
    // Test Case 1
    total++
    if runTest([]int{2, 7, 11, 15}, 9, []int{0, 1}, "Test Case 1") {
        passed++
    }
    
    fmt.Printf("\n--- Test Summary ---\n")
    fmt.Printf("Passed: %d/%d\n", passed, total)
}"#
        },
        "Zig" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

const std = @import("std");

pub fn twoSum(allocator: std.mem.Allocator, nums: []const i32, target: i32) ![]i32 {
    // TODO: implement your solution here
    _ = allocator;
    _ = nums;
    _ = target;
    return &[_]i32{};
}

fn runTest(allocator: std.mem.Allocator, nums: []const i32, target: i32, expected: []const i32, test_name: []const u8) !bool {
    const result = try twoSum(allocator, nums, target);
    defer allocator.free(result);
    
    if (std.mem.eql(i32, result, expected)) {
        std.debug.print("{s}: PASS\n", .{test_name});
        return true;
    } else {
        std.debug.print("{s}: FAIL\n", .{test_name});
        return false;
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var passed: u32 = 0;
    var total: u32 = 0;
    
    // Test Case 1
    total += 1;
    if (try runTest(allocator, &[_]i32{2, 7, 11, 15}, 9, &[_]i32{0, 1}, "Test Case 1")) {
        passed += 1;
    }
    
    std.debug.print("\n--- Test Summary ---\n");
    std.debug.print("Passed: {d}/{d}\n", .{passed, total});
}"#
        },
        "Nim" => {
            r#"# Two Sum
# Given an array of integers nums and an integer target,
# return indices of two numbers that add up to target.
#
# Example 1:
# Input: nums = [2,7,11,15], target = 9
# Output: [0,1]

proc twoSum(nums: seq[int], target: int): seq[int] =
  # TODO: implement your solution here
  result = @[]

proc runTest(nums: seq[int], target: int, expected: seq[int], testName: string): bool =
  let result = twoSum(nums, target)
  if result == expected:
    echo testName & ": PASS"
    return true
  else:
    echo testName & ": FAIL"
    return false

when isMainModule:
  var passed = 0
  var total = 0
  
  # Test Case 1
  total += 1
  if runTest(@[2, 7, 11, 15], 9, @[0, 1], "Test Case 1"):
    passed += 1
  
  echo "\n--- Test Summary ---"
  echo "Passed: ", passed, "/", total"#
        },
        _ => {
            r#"// Two Sum (Generic Example)
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]

// Function signature will depend on the specific language
// TODO: implement your solution here

// Test runner implementation will depend on the specific language
// Should include at least 5 test cases with PASS/FAIL output"#
        }
    }
}

fn run_tests() -> bool {
    let test_cases = vec![
        (vec![2,7,11,15], 9, vec![0,1]),
        (vec![3,2,4], 6, vec![1,2]),
        (vec![3,3], 6, vec![0,1])
    ];
    
    let mut passed = 0;
    for (i, (nums, target, expected)) in test_cases.iter().enumerate() {
        let result = two_sum(nums.clone(), *target);
        if result == *expected || (result.len() == expected.len() && 
           result.iter().all(|x| expected.contains(x))) {
            println!("✓ Test {} PASSED", i + 1);
            passed += 1;
        } else {
            println!("✗ Test {} FAILED: expected {:?}, got {:?}", i + 1, expected, result);
        }
    }
    
    println!("\nResult: {}/{} tests passed", passed, test_cases.len());
    passed == test_cases.len()
}

fn main() {
    run_tests();
}"#
        },
        "JavaScript" => {
            r#"// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.

function twoSum(nums, target) {
    // TODO: implement your solution here
    return [];
}

function runTests() {
    const testCases = [
        [[2,7,11,15], 9, [0,1]],
        [[3,2,4], 6, [1,2]],
        [[3,3], 6, [0,1]]
    ];
    
    let passed = 0;
    testCases.forEach((testCase, i) => {
        const [nums, target, expected] = testCase;
        try {
            const result = twoSum(nums, target);
            if (JSON.stringify(result.sort()) === JSON.stringify(expected.sort())) {
                console.log(`✓ Test ${i+1} PASSED`);
                passed++;
            } else {
                console.log(`✗ Test ${i+1} FAILED: expected ${JSON.stringify(expected)}, got ${JSON.stringify(result)}`);
            }
        } catch (e) {
            console.log(`✗ Test ${i+1} ERROR: ${e.message}`);
        }
    });
    
    console.log(`\nResult: ${passed}/${testCases.length} tests passed`);
    return passed === testCases.length;
}

runTests();"#
        },
        _ => {
            "// Generic skeleton - language-specific examples will be generated"
        }
    }
}