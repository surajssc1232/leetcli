use anyhow::Result;
use clap::Parser;
use inquire::{Select, Password};
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Parser)]
#[command(name = "leetcli")]
#[command(about = "Generate LeetCode problems with skeleton code")]
#[command(version = "0.1.0")]
struct Cli {
    /// Gemini model to use
    #[arg(short = 'm', long = "model", default_value = "gemini-2.5-flash-lite")]
    model: String,
    
    /// Problem difficulty level (interactive if no value provided)
    #[arg(short = 'd', long = "difficulty", value_name = "LEVEL")]
    difficulty: Option<String>,
    
    /// Show available models
    #[arg(long = "list-models")]
    list_models: bool,
    
    /// Show available difficulty levels
    #[arg(long = "list-difficulties")]
    list_difficulties: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.list_models {
        print_available_models();
        return Ok(());
    }
    
    if cli.list_difficulties {
        print_available_difficulties();
        return Ok(());
    }
    
    // Determine difficulty to use
    let difficulty = match cli.difficulty {
        Some(diff) => {
            // User provided -d with value
            if !is_valid_difficulty(&diff) {
                eprintln!("[!] Invalid difficulty '{}'. Use --list-difficulties to see available options.", diff);
                return Ok(());
            }
            // Update saved preference
            if let Err(e) = save_difficulty_preference(&diff) {
                eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
            }
            diff
        },
        None => {
            // Check if -d flag was used without value
            if args_contain_difficulty_flag() {
                // Show interactive dropdown
                let difficulties = vec!["easy", "medium", "hard"];
                let selected = Select::new("Select difficulty level:", difficulties).prompt()?;
                if let Err(e) = save_difficulty_preference(selected) {
                    eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
                } else {
                    println!("[+] Difficulty preference saved: {}", selected);
                }
                selected.to_string()
            } else {
                // Use saved preference or ask for first time
                get_difficulty_preference()?
            }
        }
    };
    
    println!(">> LeetCli - Generate LeetCode Problems");
    println!("Using model: {}", cli.model);
    println!("Difficulty: {}", difficulty);
    
    let api_key = get_api_key()?;
    
    let topics = vec![
        "Arrays",
        "Linked Lists", 
        "Graphs",
        "Trees",
        "Tries",
        "Dynamic Programming",
        "Recursion",
        "Hash Tables",
        "Stacks & Queues",
        "Binary Search",
        "Sorting",
        "Greedy",
        "Backtracking",
        "Bit Manipulation",
        "Two Pointers",
        "Sliding Window"
    ];

    let languages = vec![
        "Rust",
        "Python", 
        "JavaScript",
        "Java",
        "C++",
        "Go",
        "TypeScript"
    ];

    let selected_topic = Select::new("Select a topic:", topics)
        .prompt()?;

    let selected_language = Select::new("Select programming language:", languages)
        .prompt()?;

    println!("Generating {} problem for {}...", selected_topic, selected_language);

    let problem = generate_problem(&selected_topic, &selected_language, &api_key, &cli.model, &difficulty).await?;
    
    let filename = create_problem_file(&problem, &selected_language)?;
    
    println!("[✓] Problem generated: {}", filename);
    
    Ok(())
}

fn args_contain_difficulty_flag() -> bool {
    let args: Vec<String> = std::env::args().collect();
    args.iter().any(|arg| arg == "-d" || arg == "--difficulty")
}

fn print_available_models() {
    println!("Available Gemini models:");
    println!("  gemini-2.5-flash-lite    (default - fastest)");
    println!("  gemini-2.0-flash-exp     (experimental fast)");
    println!("  gemini-1.5-flash-latest  (stable fast)");
    println!("  gemini-1.5-pro-latest    (more capable)");
    println!("  gemini-pro               (older stable)");
}

fn print_available_difficulties() {
    println!("Available difficulty levels:");
    println!("  easy     - Simple problems, basic algorithms");
    println!("  medium   - Moderate complexity (default)");
    println!("  hard     - Complex problems, advanced algorithms");
}

fn is_valid_difficulty(difficulty: &str) -> bool {
    matches!(difficulty.to_lowercase().as_str(), "easy" | "medium" | "hard")
}

fn get_api_key_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    
    Ok(PathBuf::from(home_dir).join(".leetcli_api_key"))
}

fn get_difficulty_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    
    Ok(PathBuf::from(home_dir).join(".leetcli_difficulty"))
}

fn save_difficulty_preference(difficulty: &str) -> Result<()> {
    let file_path = get_difficulty_file_path()?;
    fs::write(&file_path, difficulty)?;
    
    // Set file permissions to be readable only by owner (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o600); // rw-------
        fs::set_permissions(&file_path, perms)?;
    }
    
    Ok(())
}

fn load_saved_difficulty() -> Option<String> {
    let file_path = get_difficulty_file_path().ok()?;
    if file_path.exists() {
        fs::read_to_string(&file_path).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

fn get_difficulty_preference() -> Result<String> {
    // Check saved preference first
    if let Some(saved_difficulty) = load_saved_difficulty() {
        if is_valid_difficulty(&saved_difficulty) {
            return Ok(saved_difficulty);
        }
    }
    
    // First time - ask user and save preference
    println!("[!] No difficulty preference found");
    let difficulties = vec!["easy", "medium", "hard"];
    let selected = Select::new("Select default difficulty level:", difficulties)
        .prompt()?;
    
    if let Err(e) = save_difficulty_preference(selected) {
        eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
    } else {
        println!("[+] Difficulty preference saved: {}", selected);
    }
    
    Ok(selected.to_string())
}

fn save_api_key(api_key: &str) -> Result<()> {
    let key_file = get_api_key_file_path()?;
    fs::write(&key_file, api_key)?;
    
    // Set file permissions to be readable only by owner (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&key_file)?.permissions();
        perms.set_mode(0o600); // rw-------
        fs::set_permissions(&key_file, perms)?;
    }
    
    Ok(())
}

fn load_saved_api_key() -> Option<String> {
    let key_file = get_api_key_file_path().ok()?;
    if key_file.exists() {
        fs::read_to_string(&key_file).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

fn get_api_key() -> Result<String> {
    // First check environment variable
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        if !key.trim().is_empty() {
            println!("[✓] Using Gemini API key from environment variable");
            return Ok(key);
        }
    }
    
    // Then check saved file
    if let Some(saved_key) = load_saved_api_key() {
        if !saved_key.is_empty() {
            println!("[✓] Using saved Gemini API key");
            return Ok(saved_key);
        }
    }
    
    // Finally prompt user
    println!("[!] Gemini API key not found");
    println!("Please get your API key from: https://makersuite.google.com/app/apikey");
    
    let api_key = Password::new("Enter your Gemini API key:")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()?;
    
    if api_key.trim().is_empty() {
        return Err(anyhow::anyhow!("API key cannot be empty"));
    }
    
    // Save the key for future use
    if let Err(e) = save_api_key(&api_key) {
        eprintln!("[!] Warning: Could not save API key: {}", e);
        eprintln!("You'll need to enter it again next time.");
    } else {
        println!("[+] API key saved for future use");
    }
    
    Ok(api_key)
}

async fn generate_problem(topic: &str, language: &str, api_key: &str, model: &str, difficulty: &str) -> Result<String> {
    let client = Client::new();
    
    let prompt = format!(
        "CRITICAL: The very first line must be the problem title as a comment.
Format: // Problem Title Here (for most languages) or # Problem Title Here (for Python)

Generate ONLY a LeetCode-style function skeleton for a {} problem in {} with {} difficulty.

DIFFICULTY GUIDELINES:
- Easy: Simple logic, basic data structures, straightforward algorithms
- Medium: Moderate complexity, multiple steps, common algorithms like binary search, DFS/BFS
- Hard: Complex algorithms, advanced data structures, optimization problems

EXACT FORMAT REQUIRED:
1. Problem title as the very first comment line
2. Multiple detailed examples with input/output as comments (at least 2 examples)
3. Function signature with empty body containing ONLY 'TODO: implement'
4. Test cases that call the function to verify implementation
5. NO solution logic, NO working code in the main function

Example skeleton for {}:
{}

Generate a random {} {} difficulty problem following this EXACT format.
The function body must be completely empty except for the TODO comment.
Include comprehensive test cases that demonstrate the expected behavior.
Do NOT include any markdown formatting or code blocks.",
        topic, 
        language,
        difficulty,
        language,
        get_skeleton_example(language),
        topic,
        difficulty
    );

    let request_body = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 2048
        }
    });

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, api_key);
    
    let response = client
        .post(&url)
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

fn get_skeleton_example(language: &str) -> &'static str {
    match language {
        "Rust" => {
            "// Two Sum
// Given an array of integers nums and an integer target, 
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // TODO: implement
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
    }
}"
        },
        "Python" => {
            "# Two Sum
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
    # TODO: implement
    pass

if __name__ == \"__main__\":
    # Test cases
    assert two_sum([2,7,11,15], 9) == [0,1]
    assert two_sum([3,2,4], 6) == [1,2]
    assert two_sum([3,3], 6) == [0,1]
    print(\"All tests passed!\")"
        },
        "JavaScript" => {
            "// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

function twoSum(nums, target) {
    // TODO: implement
}

// Test cases
function runTests() {
    console.log(twoSum([2,7,11,15], 9)); // Expected: [0,1]
    console.log(twoSum([3,2,4], 6));     // Expected: [1,2]
    console.log(twoSum([3,3], 6));       // Expected: [0,1]
}

runTests();"
        },
        "Java" => {
            "// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

import java.util.Arrays;

public class Solution {
    public int[] twoSum(int[] nums, int target) {
        // TODO: implement
        return new int[0];
    }
    
    public static void main(String[] args) {
        Solution solution = new Solution();
        
        // Test cases
        System.out.println(Arrays.toString(solution.twoSum(new int[]{2,7,11,15}, 9))); // Expected: [0,1]
        System.out.println(Arrays.toString(solution.twoSum(new int[]{3,2,4}, 6)));     // Expected: [1,2]
        System.out.println(Arrays.toString(solution.twoSum(new int[]{3,3}, 6)));       // Expected: [0,1]
    }
}"
        },
        "C++" => {
            "// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

#include <vector>
#include <iostream>
using namespace std;

vector<int> twoSum(vector<int>& nums, int target) {
    // TODO: implement
    return {};
}

int main() {
    // Test cases
    vector<int> nums1 = {2,7,11,15};
    vector<int> result1 = twoSum(nums1, 9);
    cout << \"Test 1: [\" << result1[0] << \",\" << result1[1] << \"]\" << endl; // Expected: [0,1]
    
    vector<int> nums2 = {3,2,4};
    vector<int> result2 = twoSum(nums2, 6);
    cout << \"Test 2: [\" << result2[0] << \",\" << result2[1] << \"]\" << endl; // Expected: [1,2]
    
    return 0;
}"
        },
        "Go" => {
            "// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

package main

import \"fmt\"

func twoSum(nums []int, target int) []int {
    // TODO: implement
    return []int{}
}

func main() {
    // Test cases
    fmt.Println(twoSum([]int{2,7,11,15}, 9)) // Expected: [0 1]
    fmt.Println(twoSum([]int{3,2,4}, 6))     // Expected: [1 2]
    fmt.Println(twoSum([]int{3,3}, 6))       // Expected: [0 1]
}"
        },
        "TypeScript" => {
            "// Two Sum
// Given an array of integers nums and an integer target,
// return indices of two numbers that add up to target.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: nums[0] + nums[1] = 2 + 7 = 9
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

function twoSum(nums: number[], target: number): number[] {
    // TODO: implement
    return [];
}

// Test cases
function runTests(): void {
    console.log(twoSum([2,7,11,15], 9)); // Expected: [0,1]
    console.log(twoSum([3,2,4], 6));     // Expected: [1,2]
    console.log(twoSum([3,3], 6));       // Expected: [0,1]
}

runTests();"
        },
        _ => {
            "// Function skeleton with TODO comment and test cases"
        }
    }
}

fn clean_content(content: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Skip markdown code block markers
        if trimmed.starts_with("```") {
            continue;
        }
        
        // Skip any lines that might contain implementation logic inside functions
        if trimmed.contains("return ") && 
           !trimmed.contains("TODO") && 
           !trimmed.starts_with("//") && 
           !trimmed.starts_with("#") &&
           !trimmed.contains("return new int[0]") && // Allow Java empty return
           !trimmed.contains("return {}") &&        // Allow C++ empty return
           !trimmed.contains("return [];") &&       // Allow TS/JS empty return
           !trimmed.contains("return []int{}") {    // Allow Go empty return
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            lines.push(format!("{}// TODO: implement", indent));
            continue;
        }
        
        // Keep test cases and function signatures
        lines.push(line.to_string());
    }
    
    lines.join("\n")
}

fn extract_problem_name(content: &str) -> String {
    // Look for first comment line containing the problem title
    let comment_patterns = vec![
        Regex::new(r"^//\s*(.+)$").unwrap(),     // // Title
        Regex::new(r"^#\s*(.+)$").unwrap(),      // # Title  
    ];
    
    for line in content.lines().take(5) {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        
        for pattern in &comment_patterns {
            if let Some(captures) = pattern.captures(trimmed) {
                if let Some(title) = captures.get(1) {
                    let title_str = title.as_str().trim();
                    
                    // Accept any reasonable title (not too restrictive)
                    if title_str.len() > 3 && title_str.len() < 60 {
                        return title_str
                            .to_lowercase()
                            .replace(" ", "_")
                            .replace("-", "_")
                            .chars()
                            .filter(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                    }
                }
            }
        }
    }
    
    // Use descriptive fallback instead of timestamp
    "unknown_problem".to_string()
}

fn create_problem_file(content: &str, language: &str) -> Result<String> {
    let extension = match language {
        "Rust" => "rs",
        "Python" => "py",
        "JavaScript" => "js", 
        "Java" => "java",
        "C++" => "cpp",
        "Go" => "go",
        "TypeScript" => "ts",
        _ => "txt"
    };

    let problem_name = extract_problem_name(content);
    let filename = format!("{}.{}", problem_name, extension);
    
    if !Path::new("problems").exists() {
        fs::create_dir("problems")?;
    }
    
    let filepath = format!("problems/{}", filename);
    fs::write(&filepath, content)?;
    
    Ok(filepath)
}
