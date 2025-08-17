use anyhow::Result;
use std::process::Command;

pub fn open_in_editor(filename: &str, editor: &str) -> Result<()> {
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
                println!("[!] Available editors: nvim, helix, nano, emacs, vim, code, etc.");
                println!("[!] You can edit the file manually: {}", filename);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to open {}: {}", editor, e))
            }
        }
    }
}