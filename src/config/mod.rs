use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn get_api_key_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    
    Ok(PathBuf::from(home_dir).join(".leetcli_api_key"))
}

pub fn get_difficulty_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    
    Ok(PathBuf::from(home_dir).join(".leetcli_difficulty"))
}

pub fn save_difficulty_preference(difficulty: &str) -> Result<()> {
    let file_path = get_difficulty_file_path()?;
    fs::write(&file_path, difficulty)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&file_path, perms)?;
    }
    
    Ok(())
}

pub fn load_saved_difficulty() -> Option<String> {
    let file_path = get_difficulty_file_path().ok()?;
    if file_path.exists() {
        fs::read_to_string(&file_path).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

pub fn save_api_key(api_key: &str) -> Result<()> {
    let key_file = get_api_key_file_path()?;
    fs::write(&key_file, api_key)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&key_file)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&key_file, perms)?;
    }
    
    Ok(())
}

pub fn load_saved_api_key() -> Option<String> {
    let key_file = get_api_key_file_path().ok()?;
    if key_file.exists() {
        fs::read_to_string(&key_file).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}