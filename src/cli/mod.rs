use clap::Parser;

#[derive(Parser)]
#[command(name = "leetcli")]
#[command(about = "Generate LeetCode problems with skeleton code")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[arg(short = 'm', long = "model", default_value = "gemini-2.5-flash-lite")]
    pub model: String,
    
    #[arg(short = 'd', long = "difficulty", value_name = "LEVEL", num_args = 0..=1, default_missing_value = "")]
    pub difficulty: Option<String>,
    
    #[arg(short = 'g', long = "graph")]
    pub graph: bool,
    
    #[arg(short = 'e', long = "editor", default_value = "nvim")]
    pub editor: String,
    
    #[arg(long = "set-api-key")]
    pub set_api_key: Option<String>,
    
    #[arg(long = "list-models")]
    pub list_models: bool,
    
    #[arg(long = "list-difficulties")]
    pub list_difficulties: bool,
}

pub fn print_available_models() {
    println!("Available Gemini models:");
    println!("  gemini-2.5-flash-lite    (default - fastest)");
    println!("  gemini-2.0-flash-exp     (experimental fast)");
    println!("  gemini-1.5-flash-latest  (stable fast)");
    println!("  gemini-1.5-pro-latest    (more capable)");
    println!("  gemini-pro               (older stable)");
}

pub fn print_available_difficulties() {
    println!("Available difficulty levels:");
    println!("  easy     - Simple problems, basic algorithms");
    println!("  medium   - Moderate complexity (default)");
    println!("  hard     - Complex problems, advanced algorithms");
}

pub fn is_valid_difficulty(difficulty: &str) -> bool {
    matches!(difficulty.to_lowercase().as_str(), "easy" | "medium" | "hard")
}