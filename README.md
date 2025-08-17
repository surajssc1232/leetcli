# LeetCli

A CLI tool that generates random LeetCode problems with skeleton code in your preferred programming language.

## Features

- Interactive topic selection (Arrays, Linked Lists, Graphs, Trees, Tries, DP, etc.)
- Multiple programming language support (Rust, Python, JavaScript, Java, C++, Go, TypeScript)
- AI-generated problems with proper skeleton code and test cases
- Automatic file creation in `problems/` directory
- Customizable AI models for different speed/quality preferences
- Command line interface with help and options

## Setup

1. Get a Gemini API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

2. Build and run:
```bash
cargo build --release
cargo run
```

The tool will ask for your Gemini API key on first run and save it securely in `~/.leetcli_api_key` for future use.

You can also set it as an environment variable to skip the prompt entirely:
```bash
export GEMINI_API_KEY="your-api-key-here"
```

## Usage

### Basic Usage
```bash
leetcli
```
On first run, you'll be prompted to select a default difficulty level which will be saved for future use.

### Interactive Difficulty Selection
```bash
leetcli -d
```
Shows a dropdown menu to select difficulty level and updates your saved preference.

### Direct Difficulty Setting
```bash
leetcli --difficulty hard
leetcli -d easy
```
Sets difficulty directly and updates your saved preference.

### Combined Options
```bash
leetcli --model gemini-1.5-pro-latest --difficulty hard
leetcli -m gemini-2.0-flash-exp -d easy
```

### List Available Options
```bash
leetcli --list-models
leetcli --list-difficulties
```

### Help
```bash
leetcli --help
```

## Available Models

- `gemini-2.5-flash-lite` (default) - Fastest model, good for quick problem generation
- `gemini-2.0-flash-exp` - Experimental fast model with latest features
- `gemini-1.5-flash-latest` - Stable fast model, reliable performance
- `gemini-1.5-pro-latest` - More capable but slower, better for complex problems
- `gemini-pro` - Older stable model

## Available Difficulty Levels

- `easy` - Simple problems with basic algorithms and data structures
- `medium` - Moderate complexity with common algorithms
- `hard` - Complex problems with advanced algorithms and optimization

## Persistent Preferences

The tool saves your preferences locally:
- **API Key**: `~/.leetcli_api_key`
- **Difficulty**: `~/.leetcli_difficulty`

Preferences are automatically updated when you use command line arguments.

## API Key Priority

The tool checks for your API key in this order:
1. `GEMINI_API_KEY` environment variable
2. Saved key file (`~/.leetcli_api_key`)
3. Interactive prompt (saves for future use)

## Example Output

### First Run
```bash
$ leetcli
>> LeetCli - Generate LeetCode Problems
[!] No difficulty preference found
? Select default difficulty level: › medium
[+] Difficulty preference saved: medium
Using model: gemini-2.5-flash-lite
Difficulty: medium
[✓] Using saved Gemini API key
? Select a topic: › Arrays
? Select programming language: › Rust
Generating Arrays problem for Rust...
[✓] Problem generated: problems/two_sum.rs
```

### Subsequent Runs
```bash
$ leetcli
>> LeetCli - Generate LeetCode Problems
Using model: gemini-2.5-flash-lite
Difficulty: medium
[✓] Using saved Gemini API key
? Select a topic: › Dynamic Programming
? Select programming language: › Python
Generating Dynamic Programming problem for Python...
[✓] Problem generated: problems/coin_change.py
```

### Interactive Difficulty Change
```bash
$ leetcli -d
>> LeetCli - Generate LeetCode Problems
? Select difficulty level: › hard
[+] Difficulty preference saved: hard
Using model: gemini-2.5-flash-lite
Difficulty: hard
```

Generated files include:
- Complete problem description with examples
- Code skeleton with proper function signatures
- Test cases to verify your implementation
- All necessary imports and boilerplate

## Command Line Options

```bash
leetcli [OPTIONS]

Options:
  -m, --model <MODEL>          Gemini model to use [default: gemini-2.5-flash-lite]
  -d, --difficulty [LEVEL]     Problem difficulty level (interactive if no value)
      --list-models            Show available models
      --list-difficulties      Show available difficulty levels
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

```bash
# Generate problem with saved preferences
leetcli

# First time setup - will ask for difficulty preference
leetcli  # Shows difficulty selection dropdown

# Interactive difficulty selection
leetcli -d  # Shows dropdown to select and save new difficulty

# Direct difficulty setting
leetcli -d easy          # Sets to easy and saves preference
leetcli --difficulty hard  # Sets to hard and saves preference

# Generate easy problem with fast model
leetcli -d easy -m gemini-2.5-flash-lite

# Generate hard problem with pro model
leetcli --difficulty hard --model gemini-1.5-pro-latest

# List all available options
leetcli --list-models
leetcli --list-difficulties
```