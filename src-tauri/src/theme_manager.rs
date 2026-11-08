use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub id: String,
    pub name: String,
    pub category: String, // "dark" or "light"
    pub bg_base: String,
    pub bg_sidebar: String,
    pub bg_card: String,
    pub border_subtle: String,
    pub text_primary: String,
    pub text_muted: String,
    pub accent: String,
    pub accent_hover: String,
    pub accent_text: String,
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("kfont"))
}

fn get_themes_file() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("themes.json"))
}

pub fn load_themes() -> Vec<CustomTheme> {
    let file = match get_themes_file() {
        Some(f) => f,
        None => return Vec::new(),
    };

    if !file.exists() {
        // Create default empty themes file with an example
        if let Some(parent) = file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let initial_themes = vec![CustomTheme {
            id: "tokyo-night".to_string(),
            name: "Tokyo Night".to_string(),
            category: "dark".to_string(),
            bg_base: "#1a1b26".to_string(),
            bg_sidebar: "#16161e".to_string(),
            bg_card: "#24283b".to_string(),
            border_subtle: "#292e42".to_string(),
            text_primary: "#c0caf5".to_string(),
            text_muted: "#565f89".to_string(),
            accent: "#7aa2f7".to_string(),
            accent_hover: "#89ddff".to_string(),
            accent_text: "#15161e".to_string(),
        }];
        if let Ok(json) = serde_json::to_string_pretty(&initial_themes) {
            let _ = fs::write(&file, json);
        }
        return initial_themes;
    }

    match fs::read_to_string(&file) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_theme(theme: CustomTheme) -> Result<(), String> {
    let file = get_themes_file().ok_or_else(|| "Failed to get config path".to_string())?;
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut themes = load_themes();
    if let Some(existing) = themes.iter_mut().find(|t| t.id == theme.id) {
        *existing = theme;
    } else {
        themes.push(theme);
    }

    let json = serde_json::to_string_pretty(&themes).map_err(|e| e.to_string())?;
    fs::write(&file, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_theme(id: &str) -> Result<(), String> {
    let file = get_themes_file().ok_or_else(|| "Failed to get config path".to_string())?;
    let mut themes = load_themes();
    themes.retain(|t| t.id != id);

    let json = serde_json::to_string_pretty(&themes).map_err(|e| e.to_string())?;
    fs::write(&file, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_config_folder_path() -> Result<String, String> {
    let dir = get_config_dir().ok_or_else(|| "Failed to get config directory".to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.to_string_lossy().to_string())
}
