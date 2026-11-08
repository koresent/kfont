use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use ttf_parser::{name_id, Face};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSummary {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_size_formatted: String,
    pub face_index: u32,
    pub format: String,
    pub family: String,
    pub style: String,
    pub full_name: String,
    pub postscript_name: String,
    pub weight: u16,
    pub width: String,
    pub is_italic: bool,
    pub is_monospaced: bool,
    pub glyph_count: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontDetails {
    pub summary: FontSummary,
    pub version: Option<String>,
    pub copyright: Option<String>,
    pub trademark: Option<String>,
    pub designer: Option<String>,
    pub designer_url: Option<String>,
    pub manufacturer: Option<String>,
    pub vendor_url: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub license_url: Option<String>,
    pub units_per_em: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub codepoints: Vec<u32>,
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn get_name_string(face: &Face, target_id: u16) -> Option<String> {
    face.names()
        .into_iter()
        .filter(|n| n.name_id == target_id)
        .find_map(|n| n.to_string())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn get_default_font_dirs() -> Vec<String> {
    let mut dirs_list = Vec::new();

    #[cfg(target_os = "linux")]
    {
        dirs_list.push("/usr/share/fonts".to_string());
        dirs_list.push("/usr/local/share/fonts".to_string());
        if let Some(data_dir) = dirs::data_dir() {
            let user_fonts = data_dir.join("fonts");
            if user_fonts.exists() {
                dirs_list.push(user_fonts.to_string_lossy().to_string());
            }
        }
        if let Some(home) = dirs::home_dir() {
            let legacy_fonts = home.join(".fonts");
            if legacy_fonts.exists() {
                dirs_list.push(legacy_fonts.to_string_lossy().to_string());
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        dirs_list.push("C:\\Windows\\Fonts".to_string());
        if let Some(data_local) = dirs::data_local_dir() {
            let win_user_fonts = data_local.join("Microsoft").join("Windows").join("Fonts");
            if win_user_fonts.exists() {
                dirs_list.push(win_user_fonts.to_string_lossy().to_string());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        dirs_list.push("/System/Library/Fonts".to_string());
        dirs_list.push("/Library/Fonts".to_string());
        if let Some(home) = dirs::home_dir() {
            let mac_user_fonts = home.join("Library").join("Fonts");
            if mac_user_fonts.exists() {
                dirs_list.push(mac_user_fonts.to_string_lossy().to_string());
            }
        }
    }

    dirs_list.retain(|p| Path::new(p).exists());
    dirs_list
}

fn parse_single_face(
    data: &[u8],
    face_index: u32,
    path_str: &str,
    file_name: &str,
    file_size: u64,
    format_str: &str,
) -> Option<FontSummary> {
    let face = Face::parse(data, face_index).ok()?;

    let family = get_name_string(&face, name_id::TYPOGRAPHIC_FAMILY)
        .or_else(|| get_name_string(&face, name_id::FAMILY))
        .unwrap_or_else(|| {
            Path::new(file_name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown Family".to_string())
        });

    let style = get_name_string(&face, name_id::TYPOGRAPHIC_SUBFAMILY)
        .or_else(|| get_name_string(&face, name_id::SUBFAMILY))
        .unwrap_or_else(|| "Regular".to_string());

    let full_name = get_name_string(&face, name_id::FULL_NAME)
        .unwrap_or_else(|| format!("{} {}", family, style));

    let postscript_name = get_name_string(&face, name_id::POST_SCRIPT_NAME)
        .unwrap_or_else(|| full_name.replace(' ', "-"));

    let id = format!("{}:{}", path_str, face_index);

    Some(FontSummary {
        id,
        file_path: path_str.to_string(),
        file_name: file_name.to_string(),
        file_size,
        file_size_formatted: format_bytes(file_size),
        face_index,
        format: format_str.to_string(),
        family,
        style,
        full_name,
        postscript_name,
        weight: face.weight().to_number(),
        width: format!("{:?}", face.width()),
        is_italic: face.is_italic(),
        is_monospaced: face.is_monospaced(),
        glyph_count: face.number_of_glyphs(),
    })
}

pub fn scan_path(target_path: &Path) -> Vec<FontSummary> {
    let mut results = Vec::new();

    if target_path.is_file() {
        if let Ok(data) = fs::read(target_path) {
            let file_name = target_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let file_size = data.len() as u64;
            let path_str = target_path.to_string_lossy().to_string();

            let ext = target_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

            let format_str = if ext == "otf" {
                "OpenType"
            } else if ext == "ttc" || ext == "otc" {
                "Font Collection"
            } else if ext == "woff" {
                "WOFF"
            } else if ext == "woff2" {
                "WOFF2"
            } else {
                "TrueType"
            };

            let count = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
            for i in 0..count {
                if let Some(summary) = parse_single_face(&data, i, &path_str, &file_name, file_size, format_str) {
                    results.push(summary);
                }
            }
        }
        return results;
    }

    for entry in WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        if ext != "ttf" && ext != "otf" && ext != "ttc" && ext != "otc" && ext != "woff" {
            continue;
        }

        let format_str = if ext == "otf" {
            "OpenType"
        } else if ext == "ttc" || ext == "otc" {
            "Font Collection"
        } else if ext == "woff" {
            "WOFF"
        } else {
            "TrueType"
        };

        if let Ok(data) = fs::read(path) {
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let file_size = data.len() as u64;
            let path_str = path.to_string_lossy().to_string();

            let count = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
            for i in 0..count {
                if let Some(summary) = parse_single_face(&data, i, &path_str, &file_name, file_size, format_str) {
                    results.push(summary);
                }
            }
        }
    }

    results
}

pub fn get_font_details(path_str: &str, face_index: u32) -> Result<FontDetails, String> {
    let data = fs::read(path_str).map_err(|e| format!("Failed to read font file: {}", e))?;
    let path = Path::new(path_str);
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let file_size = data.len() as u64;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let format_str = if ext == "otf" {
        "OpenType"
    } else if ext == "ttc" || ext == "otc" {
        "Font Collection"
    } else if ext == "woff" {
        "WOFF"
    } else if ext == "woff2" {
        "WOFF2"
    } else {
        "TrueType"
    };

    let summary = parse_single_face(&data, face_index, path_str, &file_name, file_size, format_str)
        .ok_or_else(|| "Failed to parse font summary".to_string())?;

    let face = Face::parse(&data, face_index).map_err(|e| format!("Font parse error: {:?}", e))?;

    let version = get_name_string(&face, name_id::VERSION);
    let copyright = get_name_string(&face, name_id::COPYRIGHT_NOTICE);
    let trademark = get_name_string(&face, name_id::TRADEMARK);
    let designer = get_name_string(&face, name_id::DESIGNER);
    let designer_url = get_name_string(&face, name_id::DESIGNER_URL);
    let manufacturer = get_name_string(&face, name_id::MANUFACTURER);
    let vendor_url = get_name_string(&face, name_id::VENDOR_URL);
    let description = get_name_string(&face, name_id::DESCRIPTION);
    let license = get_name_string(&face, name_id::LICENSE);
    let license_url = get_name_string(&face, name_id::LICENSE_URL);

    let mut codepoints = Vec::new();
    if let Some(cmap) = face.tables().cmap {
        for sub in cmap.subtables {
            if sub.is_unicode() {
                sub.codepoints(|cp| {
                    codepoints.push(cp);
                });
                break;
            }
        }
    }
    codepoints.sort_unstable();
    codepoints.dedup();

    Ok(FontDetails {
        summary,
        version,
        copyright,
        trademark,
        designer,
        designer_url,
        manufacturer,
        vendor_url,
        description,
        license,
        license_url,
        units_per_em: face.units_per_em(),
        ascender: face.ascender(),
        descender: face.descender(),
        line_gap: face.line_gap(),
        codepoints,
    })
}

pub fn get_font_data_url(path_str: &str) -> Result<String, String> {
    let data = fs::read(path_str).map_err(|e| format!("Failed to read font file: {}", e))?;
    let ext = Path::new(path_str)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let mime = match ext.as_str() {
        "otf" => "font/otf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        _ => "font/ttf",
    };

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&data);
    Ok(format!("data:{};base64,{}", mime, base64_data))
}
