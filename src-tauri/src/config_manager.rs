use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub id: String,
    pub name: String,
    #[serde(default = "default_category")]
    pub category: String,
    pub bg_base: String,
    pub bg_sidebar: String,
    pub bg_card: String,
    #[serde(default)]
    pub border_subtle: String,
    pub text_primary: String,
    #[serde(default)]
    pub text_muted: String,
    pub accent: String,
    #[serde(default)]
    pub accent_hover: String,
    #[serde(default)]
    pub accent_text: String,
}

fn default_category() -> String {
    "dark".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_active_theme")]
    pub active: String,
    #[serde(default)]
    pub custom_themes: Vec<CustomTheme>,
}

fn default_active_theme() -> String {
    "gruvbox-dark".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    #[serde(default = "default_preview_text")]
    pub default_text: String,
    #[serde(default = "default_list_font_size")]
    pub list_font_size: u32,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
}

fn default_preview_text() -> String {
    "The quick brown fox jumps over the lazy dog".to_string()
}

fn default_list_font_size() -> u32 {
    22
}

fn default_sort_by() -> String {
    "name-asc".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderConfig {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontsConfig {
    #[serde(default = "default_true")]
    pub scan_system_fonts: bool,
    #[serde(default)]
    pub custom_folders: Vec<FolderConfig>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub preview: PreviewConfig,
    #[serde(default)]
    pub fonts: FontsConfig,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            active: default_active_theme(),
            custom_themes: vec![CustomTheme {
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
            }],
        }
    }
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            default_text: default_preview_text(),
            list_font_size: default_list_font_size(),
            sort_by: default_sort_by(),
        }
    }
}

impl Default for FontsConfig {
    fn default() -> Self {
        Self {
            scan_system_fonts: true,
            custom_folders: Vec::new(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            preview: PreviewConfig::default(),
            fonts: FontsConfig::default(),
        }
    }
}

fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("kfont"))
}

pub fn get_config_yaml_path() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("config.yaml"))
}

pub fn load_config() -> AppConfig {
    let file = match get_config_yaml_path() {
        Some(f) => f,
        None => return AppConfig::default(),
    };

    if !file.exists() {
        let default_config = AppConfig::default();
        let _ = save_config(&default_config);
        return default_config;
    }

    match fs::read_to_string(&file) {
        Ok(content) => serde_yaml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Failed to parse config.yaml: {}", e);
            AppConfig::default()
        }),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let file = get_config_yaml_path().ok_or_else(|| "Failed to get config path".to_string())?;
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let yaml = serde_yaml::to_string(config).map_err(|e| e.to_string())?;
    fs::write(&file, yaml).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_config_file_path_str() -> Result<String, String> {
    let file = get_config_yaml_path().ok_or_else(|| "Failed to get config path".to_string())?;
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if !file.exists() {
        let _ = save_config(&AppConfig::default());
    }
    Ok(file.to_string_lossy().to_string())
}
