export interface FontSummary {
  id: string;
  file_path: string;
  file_name: string;
  file_size: number;
  file_size_formatted: string;
  face_index: number;
  format: string;
  family: string;
  style: string;
  full_name: string;
  postscript_name: string;
  weight: number;
  width: string;
  is_italic: boolean;
  is_monospaced: boolean;
  glyph_count: number;
}

export interface FontFamilyGroup {
  family: string;
  familyLower: string;
  searchIndex: string;
  styles: FontSummary[];
  defaultFont: FontSummary;
  glyphCount: number;
  isMonospaced: boolean;
  formats: string[];
}

export interface FontDetails {
  summary: FontSummary;
  version: string | null;
  copyright: string | null;
  trademark: string | null;
  designer: string | null;
  designer_url: string | null;
  manufacturer: string | null;
  vendor_url: string | null;
  description: string | null;
  license: string | null;
  license_url: string | null;
  units_per_em: number;
  ascender: number;
  descender: number;
  line_gap: number;
  codepoints: number[];
}

export interface ThemeOption {
  id: string;
  name: string;
  category: 'dark' | 'light';
  previewBg: string;
  previewAccent: string;
  isCustom?: boolean;
}

export interface CustomTheme {
  id: string;
  name: string;
  category: string;
  bg_base: string;
  bg_sidebar: string;
  bg_card: string;
  border_subtle: string;
  text_primary: string;
  text_muted: string;
  accent: string;
  accent_hover: string;
  accent_text: string;
}

export interface AppConfig {
  theme: {
    active: string;
    custom_themes: CustomTheme[];
  };
  preview: {
    default_text: string;
    list_font_size: number;
    sort_by: string;
  };
  fonts: {
    scan_system_fonts: boolean;
    custom_folders: { id: string; name: string; path: string }[];
  };
}

export const THEMES: ThemeOption[] = [
  { id: 'gruvbox-dark', name: 'Gruvbox Dark', category: 'dark', previewBg: '#282828', previewAccent: '#b8bb26' },
  { id: 'gruvbox-light', name: 'Gruvbox Light', category: 'light', previewBg: '#fbf1c7', previewAccent: '#af3a03' },
  { id: 'monokai', name: 'Monokai', category: 'dark', previewBg: '#272822', previewAccent: '#a6e22e' },
  { id: 'catppuccin-mocha', name: 'Catppuccin Mocha', category: 'dark', previewBg: '#1e1e2e', previewAccent: '#cba6f7' },
  { id: 'catppuccin-latte', name: 'Catppuccin Latte', category: 'light', previewBg: '#eff1f5', previewAccent: '#8839ef' },
  { id: 'dracula', name: 'Dracula', category: 'dark', previewBg: '#282a36', previewAccent: '#bd93f9' },
  { id: 'solarized-dark', name: 'Solarized Dark', category: 'dark', previewBg: '#002b36', previewAccent: '#2aa198' },
  { id: 'solarized-light', name: 'Solarized Light', category: 'light', previewBg: '#fdf6e3', previewAccent: '#268bd2' },
];
