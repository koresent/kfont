<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { FontSummary, FontDetails, FontFamilyGroup, CustomTheme, AppConfig } from '$lib/types';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import FontList from '$lib/components/FontList.svelte';
  import DetailView from '$lib/components/DetailView.svelte';
  import SettingsModal from '$lib/components/SettingsModal.svelte';

  let allFonts = $state<FontSummary[]>([]);
  let isScanning = $state(false);
  let selectedFolderId = $state<string>('system');
  let customFolders = $state<{ id: string; name: string; path: string }[]>([]);
  let filterCategory = $state<string>('all');
  let searchQuery = $state<string>('');
  let previewText = $state<string>('The quick brown fox jumps over the lazy dog');
  let listFontSize = $state<number>(22);
  let sortBy = $state<string>('name-asc');
  let scanSystemFonts = $state(true);

  let selectedFamily = $state<string>('');
  let selectedFont = $state<FontSummary | null>(null);
  let selectedFontDetails = $state<FontDetails | null>(null);
  let isLoadingDetails = $state(false);

  let activeTheme = $state<string>('gruvbox-dark');
  let customThemes = $state<CustomTheme[]>([]);
  let isSettingsOpen = $state(false);

  // Folders list for sidebar with counts
  let sidebarFolders = $derived.by(() => {
    const list = [
      {
        id: 'system',
        name: 'System Fonts',
        path: 'System Fonts Directory',
        count: allFonts.filter((f) => !customFolders.some((cf) => f.file_path.startsWith(cf.path))).length,
      },
    ];

    for (const cf of customFolders) {
      const count = allFonts.filter((f) => f.file_path.startsWith(cf.path)).length;
      list.push({
        id: cf.id,
        name: cf.name,
        path: cf.path,
        count,
      });
    }

    return list;
  });

  // Group fonts into families for fast virtualization
  let allGroups = $derived.by(() => {
    const map = new Map<string, FontSummary[]>();
    for (const f of allFonts) {
      const famKey = f.family.trim().toLowerCase();
      if (!map.has(famKey)) {
        map.set(famKey, []);
      }
      map.get(famKey)!.push(f);
    }

    const groups: FontFamilyGroup[] = [];
    for (const [_, styles] of map) {
      styles.sort((a, b) => a.weight - b.weight || (a.is_italic ? 1 : 0) - (b.is_italic ? 1 : 0));
      const defaultFont =
        styles.find((s) => s.style.toLowerCase() === 'regular') ||
        styles.find((s) => s.weight === 400 && !s.is_italic) ||
        styles[0];

      const family = defaultFont.family;
      const familyLower = family.toLowerCase();
      const styleTerms = styles.map((s) => `${s.style} ${s.file_name} ${s.full_name}`).join(' ').toLowerCase();
      const searchIndex = `${familyLower} ${styleTerms}`;
      const formats = Array.from(new Set(styles.map((s) => s.format)));
      const isMonospaced = styles.some((s) => s.is_monospaced);
      const glyphCount = Math.max(...styles.map((s) => s.glyph_count));

      groups.push({
        family,
        familyLower,
        searchIndex,
        styles,
        defaultFont,
        glyphCount,
        isMonospaced,
        formats,
      });
    }

    return groups;
  });

  let filteredGroups = $derived.by(() => {
    let list = allGroups;

    if (selectedFolderId !== 'system') {
      const cf = customFolders.find((f) => f.id === selectedFolderId);
      if (cf) {
        list = list.filter((g) => g.defaultFont.file_path.startsWith(cf.path));
      }
    }

    if (filterCategory === 'mono') {
      list = list.filter((g) => g.isMonospaced || g.familyLower.includes('mono'));
    } else if (filterCategory === 'italic') {
      list = list.filter((g) => g.styles.some((s) => s.is_italic || s.style.toLowerCase().includes('italic')));
    } else if (filterCategory === 'serif') {
      list = list.filter((g) => {
        const name = g.familyLower;
        return name.includes('serif') && !name.includes('sans');
      });
    } else if (filterCategory === 'sans') {
      list = list.filter((g) => {
        const name = g.familyLower;
        return name.includes('sans') || (!name.includes('serif') && !name.includes('mono'));
      });
    }

    if (searchQuery.trim()) {
      const q = searchQuery.trim().toLowerCase();
      list = list.filter((g) => g.searchIndex.includes(q));
    }

    const sorted = [...list];
    if (sortBy === 'name-asc') {
      sorted.sort((a, b) => (a.familyLower < b.familyLower ? -1 : (a.familyLower > b.familyLower ? 1 : 0)));
    } else if (sortBy === 'name-desc') {
      sorted.sort((a, b) => (a.familyLower > b.familyLower ? -1 : (a.familyLower < b.familyLower ? 1 : 0)));
    } else if (sortBy === 'styles-desc') {
      sorted.sort((a, b) => b.styles.length - a.styles.length);
    } else if (sortBy === 'glyphs-desc') {
      sorted.sort((a, b) => b.glyphCount - a.glyphCount);
    }

    return sorted;
  });

  // Styles belonging to currently selected font's family
  let currentFamilyStyles = $derived.by(() => {
    if (!selectedFont) return [];
    const grp = allGroups.find((g) => g.family.toLowerCase() === selectedFont!.family.toLowerCase());
    return grp ? grp.styles : [selectedFont];
  });

  onMount(async () => {
    try {
      // Load declarative YAML config from ~/.config/kfont/config.yaml
      const config = await invoke<AppConfig>('get_app_config');

      if (config.theme) {
        customThemes = config.theme.custom_themes || [];
        activeTheme = config.theme.active || 'gruvbox-dark';
        const custom = customThemes.find((t) => t.id === activeTheme);
        applyTheme(activeTheme, custom);
      }

      if (config.preview) {
        if (config.preview.default_text) previewText = config.preview.default_text;
        if (config.preview.list_font_size) listFontSize = config.preview.list_font_size;
        if (config.preview.sort_by) sortBy = config.preview.sort_by;
      }

      if (config.fonts) {
        scanSystemFonts = config.fonts.scan_system_fonts !== false;
        if (config.fonts.custom_folders) {
          customFolders = config.fonts.custom_folders;
        }
      }
    } catch (e) {
      console.error('Failed to load config.yaml:', e);
      applyTheme('gruvbox-dark');
    }

    await scanAllSources();
  });

  async function syncSaveConfig() {
    try {
      const cfg: AppConfig = {
        theme: {
          active: activeTheme,
          custom_themes: customThemes,
        },
        preview: {
          default_text: previewText,
          list_font_size: listFontSize,
          sort_by: sortBy,
        },
        fonts: {
          scan_system_fonts: scanSystemFonts,
          custom_folders: customFolders,
        },
      };
      await invoke('save_app_config', { config: cfg });
    } catch (e) {
      console.error('Failed to save config.yaml:', e);
    }
  }

  function applyTheme(themeId: string, customThemeData?: CustomTheme) {
    activeTheme = themeId;

    const doc = document.documentElement;

    if (customThemeData) {
      doc.removeAttribute('data-theme');
      doc.style.setProperty('--bg-base', customThemeData.bg_base);
      doc.style.setProperty('--bg-sidebar', customThemeData.bg_sidebar);
      doc.style.setProperty('--bg-card', customThemeData.bg_card);
      doc.style.setProperty('--bg-card-hover', customThemeData.border_subtle || customThemeData.bg_card);
      doc.style.setProperty('--bg-card-active', customThemeData.border_subtle || customThemeData.bg_card);
      doc.style.setProperty('--border-subtle', customThemeData.border_subtle || customThemeData.bg_card);
      doc.style.setProperty('--border-strong', customThemeData.accent);
      doc.style.setProperty('--text-primary', customThemeData.text_primary);
      doc.style.setProperty('--text-secondary', customThemeData.text_primary);
      doc.style.setProperty('--text-muted', customThemeData.text_muted || customThemeData.text_primary);
      doc.style.setProperty('--accent', customThemeData.accent);
      doc.style.setProperty('--accent-hover', customThemeData.accent_hover || customThemeData.accent);
      doc.style.setProperty('--accent-text', customThemeData.accent_text || '#1d2021');
      doc.style.setProperty('--badge-bg', customThemeData.bg_card);
      doc.style.setProperty('--badge-text', customThemeData.accent);
    } else {
      doc.style.cssText = '';
      doc.setAttribute('data-theme', themeId);
    }

    syncSaveConfig();
  }

  async function scanAllSources() {
    isScanning = true;
    try {
      const systemDirs = scanSystemFonts ? await invoke<string[]>('get_system_fonts_dirs') : [];
      const allPathsToScan = [...systemDirs, ...customFolders.map((f) => f.path)];
      const fonts = await invoke<FontSummary[]>('scan_font_folders', { paths: allPathsToScan });
      allFonts = fonts;

      // Select first family if none selected
      if (fonts.length > 0 && !selectedFont) {
        await selectFont(fonts[0]);
      }
    } catch (e) {
      console.error('Failed to scan fonts:', e);
    } finally {
      isScanning = false;
    }
  }

  async function selectFont(font: FontSummary) {
    selectedFont = font;
    selectedFamily = font.family;
    isLoadingDetails = true;
    try {
      const details = await invoke<FontDetails>('get_font_details', {
        path: font.file_path,
        faceIndex: font.face_index,
      });
      selectedFontDetails = details;
    } catch (e) {
      console.error('Failed to fetch font details:', e);
      selectedFontDetails = null;
    } finally {
      isLoadingDetails = false;
    }
  }

  function handleSelectGroup(grp: FontFamilyGroup) {
    selectedFamily = grp.family;
    selectFont(grp.defaultFont);
  }

  async function handleAddFolder() {
    try {
      const chosenPath = await invoke<string | null>('pick_folder');
      if (chosenPath && !customFolders.some((f) => f.path === chosenPath)) {
        const folderName = chosenPath.split(/[\\/]/).filter(Boolean).pop() || chosenPath;
        const newFolder = {
          id: `custom_${Date.now()}`,
          name: folderName,
          path: chosenPath,
        };
        customFolders = [...customFolders, newFolder];
        await syncSaveConfig();
        selectedFolderId = newFolder.id;
        await scanAllSources();
      }
    } catch (e) {
      console.error('Failed to add folder:', e);
    }
  }

  async function handleAddFiles() {
    try {
      const chosenFiles = await invoke<string[]>('pick_font_files');
      if (chosenFiles && chosenFiles.length > 0) {
        isScanning = true;
        const newFonts = await invoke<FontSummary[]>('scan_font_folders', { paths: chosenFiles });
        const existingIds = new Set(allFonts.map((f) => f.id));
        const merged = [...allFonts];
        for (const f of newFonts) {
          if (!existingIds.has(f.id)) {
            merged.push(f);
            existingIds.add(f.id);
          }
        }
        allFonts = merged;
        if (newFonts.length > 0) {
          await selectFont(newFonts[0]);
        }
      }
    } catch (e) {
      console.error('Failed to add font files:', e);
    } finally {
      isScanning = false;
    }
  }

  async function handleRemoveFolder(folderId: string) {
    customFolders = customFolders.filter((f) => f.id !== folderId);
    await syncSaveConfig();
    if (selectedFolderId === folderId) {
      selectedFolderId = 'system';
    }
    scanAllSources();
  }

  async function handleSaveDefaultPreviewText(text: string) {
    previewText = text;
    await syncSaveConfig();
    isSettingsOpen = false;
  }

  async function handleSaveCustomTheme(theme: CustomTheme) {
    const existingIdx = customThemes.findIndex((t) => t.id === theme.id);
    if (existingIdx >= 0) {
      customThemes[existingIdx] = theme;
    } else {
      customThemes = [...customThemes, theme];
    }
    await syncSaveConfig();
  }

  async function handleDeleteCustomTheme(id: string) {
    customThemes = customThemes.filter((t) => t.id !== id);
    if (activeTheme === id) {
      applyTheme('gruvbox-dark');
    }
    await syncSaveConfig();
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === 'Escape' && isSettingsOpen) {
      isSettingsOpen = false;
    }
  }}
/>

<div class="flex h-screen w-screen overflow-hidden">
  <!-- Left Sidebar (Folders, Categories, Settings Trigger) -->
  <Sidebar
    folders={sidebarFolders}
    {selectedFolderId}
    {filterCategory}
    totalFontsCount={allFonts.length}
    filteredCount={filteredGroups.reduce((acc, g) => acc + g.styles.length, 0)}
    {isScanning}
    onSelectFolder={(id) => (selectedFolderId = id)}
    onAddFolder={handleAddFolder}
    onAddFiles={handleAddFiles}
    onRemoveFolder={handleRemoveFolder}
    onSelectCategory={(cat) => (filterCategory = cat)}
    onRefresh={scanAllSources}
    onOpenSettings={() => (isSettingsOpen = true)}
  />

  <!-- Middle Pane (Family Groups List with Virtualized 60 FPS Scrolling) -->
  <FontList
    groups={filteredGroups}
    {selectedFamily}
    selectedFontId={selectedFont?.id || ''}
    {searchQuery}
    {previewText}
    fontSize={listFontSize}
    {sortBy}
    isLoading={isScanning}
    onSelectGroup={handleSelectGroup}
    onSearchChange={(q) => (searchQuery = q)}
    onPreviewTextChange={(t) => {
      previewText = t;
      syncSaveConfig();
    }}
    onFontSizeChange={(s) => {
      listFontSize = s;
      syncSaveConfig();
    }}
    onSortChange={(sort) => {
      sortBy = sort;
      syncSaveConfig();
    }}
  />

  <!-- Right Pane (Detailed View: Specimen, Waterfall, Glyphs, OpenType, Properties, Tester) -->
  <DetailView
    font={selectedFont}
    details={selectedFontDetails}
    familyStyles={currentFamilyStyles}
    {isLoadingDetails}
    onSelectStyle={selectFont}
  />

  <!-- Settings & Theme Modal (with Custom Themes Manager) -->
  <SettingsModal
    isOpen={isSettingsOpen}
    {activeTheme}
    defaultPreviewText={previewText}
    {customThemes}
    onClose={() => (isSettingsOpen = false)}
    onSelectTheme={(id, custom) => applyTheme(id, custom)}
    onSaveDefaultText={handleSaveDefaultPreviewText}
    onSaveCustomTheme={handleSaveCustomTheme}
    onDeleteCustomTheme={handleDeleteCustomTheme}
  />
</div>
