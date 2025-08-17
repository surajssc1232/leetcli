use anyhow::Result;
use clap::Parser;
use inquire::{Select, Password, Text};
use reqwest::Client;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use chrono::{Utc, NaiveDate, Datelike};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "leetcli")]
#[command(about = "Generate LeetCode problems with skeleton code")]
#[command(version = "0.1.0")]
struct Cli {
    /// Gemini model to use
    #[arg(short = 'm', long = "model", default_value = "gemini-2.5-flash-lite")]
    model: String,
    
    /// Problem difficulty level (interactive if no value provided)
    #[arg(short = 'd', long = "difficulty", value_name = "LEVEL", num_args = 0..=1, default_missing_value = "")]
    difficulty: Option<String>,
    
    /// Show activity graph
    #[arg(short = 'g', long = "graph")]
    graph: bool,
    
    /// Editor to use for opening problems (neovim, helix, nano, emacs, etc.)
    #[arg(short = 'e', long = "editor", default_value = "nvim")]
    editor: String,
    
    /// Set Gemini API key persistently
    #[arg(long = "set-api-key")]
    set_api_key: Option<String>,
    
    /// Show available models
    #[arg(long = "list-models")]
    list_models: bool,
    
    /// Show available difficulty levels
    #[arg(long = "list-difficulties")]
    list_difficulties: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct ActivityTracker {
    daily_counts: HashMap<String, ActivitySummary>, // date -> activity summary
    total_problems: u32,
    total_attempted: u32,
    total_solved: u32,
    streak_current: u32,
    streak_longest: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct ActivitySummary {
    attempted: u32,
    solved: u32,
}

#[derive(Serialize, Deserialize)]
struct ProblemExecution {
    filename: String,
    language: String,
    test_command: String,
    initial_hash: String,
}

#[derive(Debug)]
enum ActivityResult {
    NotAttempted,
    Attempted,
    Solved,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if let Some(api_key) = cli.set_api_key {
        if api_key.trim().is_empty() {
            eprintln!("[!] API key cannot be empty");
            return Ok(());
        }
        
        if let Err(e) = save_api_key(&api_key) {
            eprintln!("[!] Failed to save API key: {}", e);
            return Ok(());
        }
        
        println!("[+] API key saved successfully");
        println!("[\u{1F4A1}] You can now use LeetCli without entering your API key each time");
        return Ok(());
    }
    
    if cli.graph {
        show_activity_graph()?;
        return Ok(());
    }
    
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
        Some(diff) if !diff.is_empty() => {
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
        Some(_) => {
            // -d flag used without value (empty string) - show interactive dropdown and exit
            let difficulties = vec!["easy", "medium", "hard"];
            let selected = Select::new("Select difficulty level:", difficulties).prompt()?;
            if let Err(e) = save_difficulty_preference(selected) {
                eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
            } else {
                println!("[+] Difficulty preference saved: {}", selected);
            }
            return Ok(()); // Exit after setting difficulty
        },
        None => {
            // No -d flag used - use saved preference or ask for first time
            get_difficulty_preference()?
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
        // Popular/Mainstream
        "Python",
        "JavaScript", 
        "Java",
        "C++",
        "C#",
        "TypeScript",
        "Go",
        "Rust",
        "PHP",
        "Swift",
        "Kotlin",
        "Ruby",
        "C",
        // Systems/Low-level
        "Assembly",
        "Zig",
        "D",
        "Nim",
        "Crystal",
        "V",
        // Functional
        "Haskell",
        "OCaml",
        "F#",
        "Erlang",
        "Elixir",
        "Clojure",
        "Scala",
        "Lisp",
        "Scheme",
        "ML",
        // JVM Languages
        "Groovy",
        "Ceylon",
        // .NET Languages
        "VB.NET",
        "PowerShell",
        // Web Technologies
        "Dart",
        "CoffeeScript",
        "Elm",
        "PureScript",
        "ReasonML",
        // Data Science/Math
        "R",
        "Julia",
        "MATLAB",
        "Octave",
        "Mathematica",
        "SAS",
        "SPSS",
        // Legacy/Enterprise
        "COBOL",
        "Fortran",
        "Pascal",
        "Ada",
        "Delphi",
        "Visual Basic",
        "BASIC",
        "PL/SQL",
        // Scripting
        "Perl",
        "Lua",
        "Tcl",
        "AWK",
        "Bash",
        "Fish",
        "Zsh",
        // Mobile
        "Objective-C",
        "Flutter",
        "React Native",
        // Game Development
        "GDScript",
        "UnityScript",
        "Blueprints",
        // Educational
        "Scratch",
        "Logo",
        "Alice",
        // Esoteric
        "Brainfuck",
        "Whitespace",
        "Malbolge",
        "Befunge",
        // Database
        "SQL",
        "NoSQL",
        // Configuration/Markup
        "JSON",
        "XML", 
        "YAML",
        "TOML",
        // Other
        "Prolog",
        "Smalltalk",
        "APL",
        "J",
        "K",
        "Q",
        "Factor",
        "Forth",
        "PostScript",
        "LaTeX",
        "Custom (specify your own)"
    ];

    let selected_topic = Select::new("Select a topic:", topics)
        .prompt()?;

    let selected_language_option = Select::new("Select programming language:", languages)
        .prompt()?;

    // Handle custom language input
    let selected_language = if selected_language_option == "Custom (specify your own)" {
        Text::new("Enter your programming language:")
            .with_help_message("e.g., Python, JavaScript, Assembly, etc.")
            .prompt()?
    } else {
        selected_language_option.to_string()
    };

    println!("Generating {} problem for {}...", selected_topic, selected_language);

    let problem = generate_problem_with_tests(&selected_topic, &selected_language, &api_key, &cli.model, &difficulty).await?;
    
    let filename = create_problem_file(&problem, &selected_language)?;
    
    println!("[✓] Problem generated: {}", filename);
    
    // Calculate initial file hash for change detection
    let initial_hash = calculate_file_hash(&filename)?;
    
    // Generate test command for the language
    let test_command = generate_test_command(&selected_language, &filename);
    
    // Save execution info for testing later
    let exec_info = ProblemExecution {
        filename: filename.clone(),
        language: selected_language.clone(),
        test_command: test_command.clone(),
        initial_hash: initial_hash.clone(),
    };
    save_execution_info(&exec_info)?;
    
    println!("[\u{1F4DD}] Opening in {}...", cli.editor);
    println!("[\u{1F4A1}] After solving, save and exit to automatically validate your solution");
    
    // Open in specified editor and wait for it to close
    open_in_editor(&filename, &cli.editor)?;
    
    // Track activity based on file changes and test results
    let activity_result = track_enhanced_activity(&filename, &initial_hash, &test_command)?;
    
    match activity_result {
        ActivityResult::Solved => {
            println!("[\u{2705}] All tests passed! Problem solved!");
            record_activity_completion(&ActivityResult::Solved)?;
        }
        ActivityResult::Attempted => {
            println!("[\u{1F4DD}] Good effort! You modified the code but some tests failed.");
            println!("[\u{1F4A1}] Keep practicing! Run tests manually: {}", test_command);
            record_activity_completion(&ActivityResult::Attempted)?;
        }
        ActivityResult::NotAttempted => {
            println!("[\u{1F914}] No changes detected. Try solving the problem next time!");
        }
    }
    
    show_daily_progress()?;
    
    Ok(())
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

async fn generate_problem_with_tests(topic: &str, language: &str, api_key: &str, model: &str, difficulty: &str) -> Result<String> {
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

fn get_activity_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    Ok(PathBuf::from(home_dir).join(".leetcli_activity.json"))
}

fn get_execution_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    Ok(PathBuf::from(home_dir).join(".leetcli_last_execution.json"))
}

fn load_activity_tracker() -> Result<ActivityTracker> {
    let path = get_activity_file_path()?;
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(ActivityTracker::default())
    }
}

fn save_activity_tracker(tracker: &ActivityTracker) -> Result<()> {
    let path = get_activity_file_path()?;
    let content = serde_json::to_string_pretty(tracker)?;
    fs::write(path, content)?;
    Ok(())
}

fn save_execution_info(exec_info: &ProblemExecution) -> Result<()> {
    let path = get_execution_file_path()?;
    let content = serde_json::to_string_pretty(exec_info)?;
    fs::write(path, content)?;
    Ok(())
}

fn generate_test_command(language: &str, filename: &str) -> String {
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

fn open_in_editor(filename: &str, editor: &str) -> Result<()> {
    let status = Command::new(editor)
        .arg(filename)
        .status();
    
    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("{} exited with error", editor))
            }
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("[!] {} not found. Please install {} or use a different editor with -e/--editor", editor, editor);
                println!("[\u{1F4A1}] Available editors: nvim, helix, nano, emacs, vim, code, etc.");
                println!("[\u{1F4A1}] You can edit the file manually: {}", filename);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to open {}: {}", editor, e))
            }
        }
    }
}

fn show_activity_graph() -> Result<()> {
    let tracker = load_activity_tracker()?;
    
    // Show current month
    let now = Utc::now().date_naive();
    let current_year = now.year();
    let current_month = now.month();
    
    // Get the first and last day of current month
    let first_day = NaiveDate::from_ymd_opt(current_year, current_month, 1)
        .ok_or_else(|| anyhow::anyhow!("Invalid first day of month"))?;
    
    let last_day = if current_month == 12 {
        NaiveDate::from_ymd_opt(current_year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(current_year, current_month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };
    
    let month_name = match current_month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown",
    };
    
    println!("{} {}", month_name, current_year);
    println!();
    
    // Calculate the calendar grid - start from Monday of the week containing the 1st
    let mut calendar_start = first_day;
    while calendar_start.weekday().num_days_from_monday() != 0 {
        calendar_start = calendar_start.pred_opt().unwrap();
    }
    
    // End at Sunday of the week containing the last day
    let mut calendar_end = last_day;
    while calendar_end.weekday().num_days_from_monday() != 6 {
        calendar_end = calendar_end.succ_opt().unwrap();
    }
    
    let total_days = (calendar_end - calendar_start).num_days() + 1;
    let total_weeks = (total_days / 7) as usize;
    
    // Print week starting dates only
    print!("     ");
    for week in 0..total_weeks {
        let week_start = calendar_start + chrono::Duration::days(week as i64 * 7);
        if week_start.month() == current_month {
            print!("{:>3}", week_start.day());
        } else {
            print!("   ");
        }
    }
    println!();
    
    // Print the activity grid
    let week_days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (day_of_week, day_label) in week_days.iter().enumerate() {
        print!("{:<3}  ", day_label);
        
        for week in 0..total_weeks {
            let date = calendar_start + chrono::Duration::days(week as i64 * 7 + day_of_week as i64);
            
            if date.month() == current_month && date.year() == current_year {
                let date_str = date.format("%Y-%m-%d").to_string();
                let default_summary = ActivitySummary::default();
                let summary = tracker.daily_counts.get(&date_str).unwrap_or(&default_summary);
                
                // Different symbols based on activity type
                let (_symbol, colored_symbol) = if summary.solved > 0 {
                    ("■", "■".green()) // Solved = green filled
                } else if summary.attempted > 0 {
                    ("▣", "▣".yellow()) // Attempted = yellow outline
                } else {
                    ("□", "□".dimmed()) // No activity = dimmed empty
                };
                
                print!("{:>3}", colored_symbol);
            } else {
                // Days outside current month
                print!("   ");
            }
        }
        println!();
    }
    
    println!();
    print!("Less ");
    print!("{}", "□".dimmed()); // no activity
    print!(" ");
    print!("{}", "▣".yellow());  // attempted
    print!(" ");
    print!("{}", "■".green());  // solved
    println!(" More");
    
    Ok(())
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

fn calculate_file_hash(filename: &str) -> Result<String> {
    let content = fs::read(filename)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn track_enhanced_activity(filename: &str, initial_hash: &str, test_command: &str) -> Result<ActivityResult> {
    // Check if file was modified
    let final_hash = calculate_file_hash(filename)?;
    
    if initial_hash == final_hash {
        return Ok(ActivityResult::NotAttempted);
    }
    
    // File was modified, now check if tests pass
    println!("[\u{1F50D}] Code changes detected! Running validation tests...");
    
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
            
            // Print output for debugging
            if !stdout.is_empty() {
                println!("{}", stdout);
            }
            if !stderr.is_empty() && !stderr.trim().is_empty() {
                println!("Warnings: {}", stderr);
            }
            
            // Check for test success indicators in output
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
            
            // Check for explicit failure first
            if failure_indicators.iter().any(|&indicator| output_text.contains(&indicator.to_lowercase())) {
                return Ok(false);
            }
            
            // Check for success indicators
            if success_indicators.iter().any(|&indicator| output_text.contains(&indicator.to_lowercase())) {
                return Ok(true);
            }
            
            // Fallback: if no explicit indicators, check exit code
            Ok(result.status.success())
        },
        Err(e) => {
            println!("[\u{274C}] Failed to run tests: {}", e);
            Ok(false)
        }
    }
}

fn record_activity_completion(activity: &ActivityResult) -> Result<()> {
    let mut tracker = load_activity_tracker()?;
    let today = Utc::now().format("%Y-%m-%d").to_string();
    
    // Get or create today's summary
    let summary = tracker.daily_counts.entry(today.clone()).or_insert(ActivitySummary::default());
    
    match activity {
        ActivityResult::Attempted => {
            summary.attempted += 1;
            tracker.total_attempted += 1;
        }
        ActivityResult::Solved => {
            summary.solved += 1;
            tracker.total_solved += 1;
            tracker.total_problems += 1;
        }
        ActivityResult::NotAttempted => {
            // No tracking for not attempted
        }
    }
    
    // Update streaks based on any activity (attempted or solved)
    update_enhanced_streaks(&mut tracker)?;
    
    save_activity_tracker(&tracker)?;
    Ok(())
}

fn update_enhanced_streaks(tracker: &mut ActivityTracker) -> Result<()> {
    let mut dates: Vec<NaiveDate> = tracker.daily_counts.keys()
        .filter_map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
        .filter(|date| {
            if let Some(summary) = tracker.daily_counts.get(&date.format("%Y-%m-%d").to_string()) {
                summary.attempted > 0 || summary.solved > 0
            } else {
                false
            }
        })
        .collect();
    dates.sort();
    
    if dates.is_empty() {
        return Ok(());
    }
    
    let today = Utc::now().date_naive();
    let mut current_streak = 0;
    let mut temp_streak = 0;
    let mut longest_streak = 0;
    
    // Calculate current streak (working backwards from today)
    let mut check_date = today;
    while let Some(summary) = tracker.daily_counts.get(&check_date.format("%Y-%m-%d").to_string()) {
        if summary.attempted > 0 || summary.solved > 0 {
            current_streak += 1;
            check_date = check_date.pred_opt().unwrap_or(check_date);
        } else {
            break;
        }
    }
    
    // Calculate longest streak
    for window in dates.windows(2) {
        if let [prev, curr] = window {
            if (*curr - *prev).num_days() == 1 {
                temp_streak += 1;
            } else {
                longest_streak = longest_streak.max(temp_streak + 1);
                temp_streak = 0;
            }
        }
    }
    longest_streak = longest_streak.max(temp_streak + 1);
    
    tracker.streak_current = current_streak;
    tracker.streak_longest = longest_streak.max(tracker.streak_longest as i64) as u32;
    
    Ok(())
}

fn show_daily_progress() -> Result<()> {
    let tracker = load_activity_tracker()?;
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let default_summary = ActivitySummary::default();
    let today_summary = tracker.daily_counts.get(&today).unwrap_or(&default_summary);
    
    println!("\n{}", "\u{1F3AF} Daily Progress".bright_cyan());
    println!("{}", "═══════════════".bright_cyan());
    
    if today_summary.solved > 0 {
        println!("\u{2705} Problems solved today: {}", today_summary.solved.to_string().bright_green());
    }
    if today_summary.attempted > 0 {
        println!("\u{1F4DD} Problems attempted today: {}", today_summary.attempted.to_string().bright_yellow());
    }
    if today_summary.solved == 0 && today_summary.attempted == 0 {
        println!("\u{1F4C5} No activity today yet - time to start!");
    }
    
    println!("\u{1F4CA} Total solved: {}", tracker.total_solved.to_string().bright_green());
    println!("\u{1F4DD} Total attempted: {}", tracker.total_attempted.to_string().bright_yellow());
    println!("\u{1F525} Current streak: {} days", tracker.streak_current.to_string().bright_red());
    println!("\u{2B50} Longest streak: {} days", tracker.streak_longest.to_string().bright_magenta());
    
    Ok(())
}
