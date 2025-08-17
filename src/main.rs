use anyhow::Result;
use clap::Parser;
use inquire::{Select, Password, Text};

mod cli;
mod api;
mod activity;
mod problem;
mod config;
mod editor;
mod test;

use cli::{Cli, print_available_models, print_available_difficulties, is_valid_difficulty};
use api::generate_problem_with_tests;
use activity::{show_activity_graph, record_activity_completion, show_daily_progress, ActivityResult};
use problem::create_problem_file;
use config::{save_difficulty_preference, save_api_key, load_saved_difficulty, load_saved_api_key};
use editor::open_in_editor;
use test::{ProblemExecution, save_execution_info, generate_test_command, calculate_file_hash, track_enhanced_activity};

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
        println!("[!] You can now use LeetCli without entering your API key each time");
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
    
    let difficulty = match cli.difficulty {
        Some(diff) if !diff.is_empty() => {
            if !is_valid_difficulty(&diff) {
                eprintln!("[!] Invalid difficulty '{}'. Use --list-difficulties to see available options.", diff);
                return Ok(());
            }
            if let Err(e) = save_difficulty_preference(&diff) {
                eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
            }
            diff
        },
        Some(_) => {
            let difficulties = vec!["easy", "medium", "hard"];
            let selected = Select::new("Select difficulty level:", difficulties).prompt()?;
            if let Err(e) = save_difficulty_preference(selected) {
                eprintln!("[!] Warning: Could not save difficulty preference: {}", e);
            } else {
                println!("[+] Difficulty preference saved: {}", selected);
            }
            return Ok(());
        },
        None => {
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
        "Assembly",
        "Zig",
        "D",
        "Nim",
        "Crystal",
        "V",
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
        "Groovy",
        "Ceylon",
        "VB.NET",
        "PowerShell",
        "Dart",
        "CoffeeScript",
        "Elm",
        "PureScript",
        "ReasonML",
        "R",
        "Julia",
        "MATLAB",
        "Octave",
        "Mathematica",
        "SAS",
        "SPSS",
        "COBOL",
        "Fortran",
        "Pascal",
        "Ada",
        "Delphi",
        "Visual Basic",
        "BASIC",
        "PL/SQL",
        "Perl",
        "Lua",
        "Tcl",
        "AWK",
        "Bash",
        "Fish",
        "Zsh",
        "Objective-C",
        "Flutter",
        "React Native",
        "GDScript",
        "UnityScript",
        "Blueprints",
        "Scratch",
        "Logo",
        "Alice",
        "Brainfuck",
        "Whitespace",
        "Malbolge",
        "Befunge",
        "SQL",
        "NoSQL",
        "JSON",
        "XML", 
        "YAML",
        "TOML",
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
    
    let initial_hash = calculate_file_hash(&filename)?;
    
    let test_command = generate_test_command(&selected_language, &filename);
    
    let exec_info = ProblemExecution {
        filename: filename.clone(),
        language: selected_language.clone(),
        test_command: test_command.clone(),
        initial_hash: initial_hash.clone(),
    };
    save_execution_info(&exec_info)?;
    
    println!("[#] Opening in {}...", cli.editor);
    println!("[!] After solving, save and exit to automatically validate your solution");
    
    open_in_editor(&filename, &cli.editor)?;
    
    let activity_result = track_enhanced_activity(&filename, &initial_hash, &test_command)?;
    
    match activity_result {
        ActivityResult::Solved => {
            println!("[✓] All tests passed! Problem solved!");
            record_activity_completion(&ActivityResult::Solved)?;
        }
        ActivityResult::Attempted => {
            println!("[#] Good effort! You modified the code but some tests failed.");
            println!("[!] Keep practicing! Run tests manually: {}", test_command);
            record_activity_completion(&ActivityResult::Attempted)?;
        }
        ActivityResult::NotAttempted => {
            println!("[?] No changes detected. Try solving the problem next time!");
        }
    }
    
    show_daily_progress()?;
    
    Ok(())
}

fn get_difficulty_preference() -> Result<String> {
    if let Some(saved_difficulty) = load_saved_difficulty() {
        if is_valid_difficulty(&saved_difficulty) {
            return Ok(saved_difficulty);
        }
    }
    
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

fn get_api_key() -> Result<String> {
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        if !key.trim().is_empty() {
            println!("[✓] Using Gemini API key from environment variable");
            return Ok(key);
        }
    }
    
    if let Some(saved_key) = load_saved_api_key() {
        if !saved_key.is_empty() {
            println!("[✓] Using saved Gemini API key");
            return Ok(saved_key);
        }
    }
    
    println!("[!] Gemini API key not found");
    println!("Please get your API key from: https://makersuite.google.com/app/apikey");
    
    let api_key = Password::new("Enter your Gemini API key:")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()?;
    
    if api_key.trim().is_empty() {
        return Err(anyhow::anyhow!("API key cannot be empty"));
    }
    
    if let Err(e) = save_api_key(&api_key) {
        eprintln!("[!] Warning: Could not save API key: {}", e);
        eprintln!("You'll need to enter it again next time.");
    } else {
        println!("[+] API key saved for future use");
    }
    
    Ok(api_key)
}