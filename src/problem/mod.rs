use anyhow::Result;
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn extract_problem_name(content: &str) -> String {
    let comment_patterns = vec![
        Regex::new(r"^//\s*(.+)$").unwrap(),
        Regex::new(r"^#\s*(.+)$").unwrap(),
    ];
    
    for line in content.lines().take(5) {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        
        for pattern in &comment_patterns {
            if let Some(captures) = pattern.captures(trimmed) {
                if let Some(title) = captures.get(1) {
                    let title_str = title.as_str().trim();
                    
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
    
    "unknown_problem".to_string()
}

pub fn create_problem_file(content: &str, language: &str) -> Result<String> {
    let extension = match language {
        "Rust" => "rs",
        "Python" | "Python3" => "py",
        "JavaScript" => "js",
        "Java" => "java",
        "C++" => "cpp",
        "C" => "c",
        "Go" => "go",
        "TypeScript" => "ts",
        "C#" | "Csharp" => "cs",
        "PHP" => "php",
        "Ruby" => "rb",
        "Swift" => "swift",
        "Kotlin" => "kt",
        "Scala" => "scala",
        "Perl" => "pl",
        "R" => "r",
        "MATLAB" => "m",
        "Dart" => "dart",
        "Elixir" => "ex",
        "Erlang" => "erl",
        "Clojure" => "clj",
        "Haskell" => "hs",
        "OCaml" => "ml",
        "F#" => "fs",
        "Lua" => "lua",
        "Shell" | "Bash" => "sh",
        "PowerShell" => "ps1",
        "SQL" => "sql",
        "HTML" => "html",
        "CSS" => "css",
        "SCSS" => "scss",
        "SASS" => "sass",
        "LESS" => "less",
        "JSON" => "json",
        "XML" => "xml",
        "YAML" => "yml",
        "TOML" => "toml",
        "Vim script" => "vim",
        "Assembly" => "asm",
        "COBOL" => "cob",
        "Fortran" => "f90",
        "Pascal" => "pas",
        "Delphi" => "pas",
        "Ada" => "adb",
        "Lisp" => "lisp",
        "Scheme" => "scm",
        "Prolog" => "pro",
        "Groovy" => "groovy",
        "Visual Basic" => "vb",
        "Objective-C" => "m",
        "D" => "d",
        "Nim" => "nim",
        "Crystal" => "cr",
        "Zig" => "zig",
        "V" => "v",
        "Julia" => "jl",
        "Racket" => "rkt",
        "Smalltalk" => "st",
        "Tcl" => "tcl",
        "AWK" => "awk",
        "SED" => "sed",
        "Makefile" => "mk",
        "CMake" => "cmake",
        "Dockerfile" => "dockerfile",
        "LaTeX" => "tex",
        "Markdown" => "md",
        "ReStructuredText" => "rst",
        "AsciiDoc" => "adoc",
        "GraphQL" => "graphql",
        "Solidity" => "sol",
        "VHDL" => "vhd",
        "Verilog" => "ver",
        "SystemVerilog" => "sv",
        "CUDA" => "cu",
        "OpenCL" => "cl",
        "HLSL" => "hlsl",
        "GLSL" => "glsl",
        "CoffeeScript" => "coffee",
        "LiveScript" => "ls",
        "PureScript" => "purs",
        "Elm" => "elm",
        "Reason" => "re",
        "ReScript" => "res",
        "Idris" => "idr",
        "Agda" => "agda",
        "Coq" => "v",
        "Lean" => "lean",
        "APL" => "apl",
        "J" => "ijs",
        "K" => "k",
        "Q" => "q",
        "BrainF*ck" => "bf",
        "Whitespace" => "ws",
        "Malbolge" => "mal",
        "Befunge" => "bf",
        "INTERCAL" => "i",
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