use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub async fn generate_problem_with_tests(topic: &str, language: &str, api_key: &str, model: &str, difficulty: &str) -> Result<String> {
    let client = Client::new();
    
    let prompt = format!(
        "CRITICAL: The very first line must be the problem title as a comment.
Format: // Problem Title Here (for most languages) or # Problem Title Here (for Python)

Generate a LeetCode-style {} problem in {} with {} difficulty that includes EMBEDDED TEST CASES.

DIFFICULTY GUIDELINES:
- Easy: Simple logic, basic data structures, straightforward algorithms
- Medium: Moderate complexity, multiple steps, common algorithms like binary search, DFS/BFS
- Hard: Complex algorithms, advanced data structures, optimization problems

EXACT FORMAT REQUIRED:
1. Problem title as the very first comment line
2. Detailed problem description with examples (at least 2-3 examples)
3. Function signature with empty body containing 'TODO: implement your solution here'
4. EMBEDDED TEST RUNNER with at least 5 diverse test cases
5. Test runner that prints PASS/FAIL for each test and final summary
6. NO solution logic in the main function - just the skeleton

Example structure for {}:
{}

CRITICAL REQUIREMENTS:
- Function body must be EMPTY (just TODO comment)
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