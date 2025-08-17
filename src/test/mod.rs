use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use sha2::{Sha256, Digest};
use crate::activity::ActivityResult;

#[derive(Serialize, Deserialize)]
pub struct ProblemExecution {
    pub filename: String,
    pub language: String,
    pub test_command: String,
    pub initial_hash: String,
}

pub fn get_execution_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    Ok(PathBuf::from(home_dir).join(".leetcli_last_execution.json"))
}

pub fn save_execution_info(exec_info: &ProblemExecution) -> Result<()> {
    let path = get_execution_file_path()?;
    let content = serde_json::to_string_pretty(exec_info)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn has_test_command(language: &str) -> bool {
    match language.to_lowercase().as_str() {
        "rust" | "python" | "javascript" | "java" | "c++" | "go" | "typescript" => true,
        _ => false,
    }
}

pub fn generate_test_command(language: &str, filename: &str) -> String {
    match language.to_lowercase().as_str() {
        "rust" => format!("cd problems && rustc {} && ./{}", filename, filename.replace(".rs", "")),
        "python" => format!("cd problems && python {}", filename.replace("problems/", "")),
        "javascript" => format!("cd problems && node {}", filename.replace("problems/", "")),
        "java" => {
            let class_name = filename.replace("problems/", "").replace(".java", "");
            format!("cd problems && javac {} && java {}", filename.replace("problems/", ""), class_name)
        },
        "c++" => {
            let exe_name = filename.replace("problems/", "").replace(".cpp", "");
            format!("cd problems && g++ {} -o {} && ./{}", filename.replace("problems/", ""), exe_name, exe_name)
        },
        "go" => format!("cd problems && go run {}", filename.replace("problems/", "")),
        "typescript" => format!("cd problems && tsc {} && node {}", filename.replace("problems/", ""), filename.replace(".ts", ".js").replace("problems/", "")),
        _ => format!("echo 'No test command available for {}'", language),
    }
}

pub fn calculate_file_hash(filename: &str) -> Result<String> {
    let content = fs::read_to_string(filename)?;
    let normalized = content
        .lines()
        .map(|line| line.trim_end())
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn has_meaningful_changes(filename: &str, _initial_hash: &str) -> Result<bool> {
    let current_content = fs::read_to_string(filename)?;
    
    let meaningful_indicators = [
        "return ",
        "=",
        "if ",
        "for ",
        "while ",
        "def ",
        "fn ",
        "function ",
        "class ",
        "{",
        "print",
        "console.log",
        "println!",
    ];
    
    let lines: Vec<&str> = current_content.lines().collect();
    let mut meaningful_lines = 0;
    
    for line in &lines {
        let trimmed = line.trim();
        
        if trimmed.is_empty() || 
           trimmed.starts_with("//") || 
           trimmed.starts_with("#") || 
           trimmed.to_lowercase().contains("todo") ||
           trimmed == "{" || trimmed == "}" ||
           trimmed == "pass" ||
           trimmed == "return [];" || trimmed == "return vec![];"
        {
            continue;
        }
        
        if meaningful_indicators.iter().any(|&indicator| trimmed.contains(indicator)) {
            meaningful_lines += 1;
        }
    }
    
    Ok(meaningful_lines >= 2)
}

pub fn track_enhanced_activity(filename: &str, initial_hash: &str, test_command: &str) -> Result<ActivityResult> {
    let final_hash = calculate_file_hash(filename)?;
    
    if initial_hash == final_hash {
        return Ok(ActivityResult::NotAttempted);
    }
    
    if !has_meaningful_changes(filename, initial_hash)? {
        return Ok(ActivityResult::NotAttempted);
    }
    
    println!("[*] Code changes detected! Running validation tests...");
    
    if run_embedded_tests(test_command)? {
        Ok(ActivityResult::Solved)
    } else {
        Ok(ActivityResult::Attempted)
    }
}

fn run_embedded_tests(test_command: &str) -> Result<bool> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", test_command])
            .output()
    } else {
        Command::new("sh")
            .args(["-c", test_command])
            .output()
    };

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);
            
            if !stdout.is_empty() {
                println!("{}", stdout);
            }
            if !stderr.is_empty() && !stderr.trim().is_empty() {
                println!("Warnings: {}", stderr);
            }
            
            let success_indicators = [
                "tests passed",
                "PASSED",
                "✓",
                "All tests passed",
                "100%"
            ];
            
            let failure_indicators = [
                "FAILED",
                "✗",
                "ERROR",
                "test failed",
                "0/"
            ];
            
            let output_text = stdout.to_lowercase();
            
            if failure_indicators.iter().any(|&indicator| output_text.contains(&indicator.to_lowercase())) {
                return Ok(false);
            }
            
            if success_indicators.iter().any(|&indicator| output_text.contains(&indicator.to_lowercase())) {
                return Ok(true);
            }
            
            Ok(result.status.success())
        },
        Err(e) => {
            println!("[×] Failed to run tests: {}", e);
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_test_command() {
        // Languages with test commands
        assert!(has_test_command("rust"));
        assert!(has_test_command("python"));
        assert!(has_test_command("javascript"));
        assert!(has_test_command("java"));
        assert!(has_test_command("c++"));
        assert!(has_test_command("go"));
        assert!(has_test_command("typescript"));
        
        // Test case insensitive
        assert!(has_test_command("RUST"));
        assert!(has_test_command("Python"));
        assert!(has_test_command("JavaScript"));
        
        // Languages without test commands
        assert!(!has_test_command("zig"));
        assert!(!has_test_command("nim"));
        assert!(!has_test_command("swift"));
        assert!(!has_test_command("kotlin"));
        assert!(!has_test_command("unknown"));
    }
}