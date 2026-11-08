<script lang="ts">
  import { X, Check, Palette, Folder, Plus, Trash2 } from '@lucide/svelte';
  import { THEMES, type ThemeOption, type CustomTheme } from '$lib/types';
  import { invoke } from '@tauri-apps/api/core';
  import { revealItemInDir } from '@tauri-apps/plugin-opener';

  let {
    isOpen = false,
    activeTheme = 'gruvbox-dark',
    defaultPreviewText = 'The quick brown fox jumps over the lazy dog',
    customThemes = [],
    onClose,
    onSelectTheme,
    onSaveDefaultText,
    onSaveCustomTheme,
    onDeleteCustomTheme,
  }: {
    isOpen: boolean;
    activeTheme: string;
    defaultPreviewText: string;
    customThemes: CustomTheme[];
    onClose: () => void;
    onSelectTheme: (themeId: string, customThemeData?: CustomTheme) => void;
    onSaveDefaultText: (text: string) => void;
    onSaveCustomTheme: (theme: CustomTheme) => Promise<void>;
    onDeleteCustomTheme: (id: string) => Promise<void>;
  } = $props();

  let tempDefaultText = $state('');
  let isCreatingCustom = $state(false);

  // New custom theme form state
  let newThemeName = $state('My Custom Theme');
  let newThemeCategory = $state<'dark' | 'light'>('dark');
  let newBgBase = $state('#1a1b26');
  let newBgSidebar = $state('#16161e');
  let newBgCard = $state('#24283b');
  let newTextPrimary = $state('#c0caf5');
  let newTextMuted = $state('#565f89');
  let newAccent = $state('#7aa2f7');

  $effect(() => {
    tempDefaultText = defaultPreviewText;
  });

  function handleSaveText() {
    onSaveDefaultText(tempDefaultText);
  }

  async function handleOpenConfigFile() {
    try {
      const configPath = await invoke<string>('get_config_yaml_file');
      await revealItemInDir(configPath);
    } catch (e) {
      console.error('Failed to open config file:', e);
    }
  }

  async function handleSaveNewCustomTheme() {
    if (!newThemeName.trim()) return;

    const id = `custom-${newThemeName.toLowerCase().replace(/[^a-z0-9]/g, '-')}-${Date.now().toString().slice(-4)}`;
    const custom: CustomTheme = {
      id,
      name: newThemeName.trim(),
      category: newThemeCategory,
      bg_base: newBgBase,
      bg_sidebar: newBgSidebar,
      bg_card: newBgCard,
      border_subtle: newBgCard,
      text_primary: newTextPrimary,
      text_muted: newTextMuted,
      accent: newAccent,
      accent_hover: newAccent,
      accent_text: newBgBase,
    };

    try {
      await onSaveCustomTheme(custom);
      onSelectTheme(custom.id, custom);
      isCreatingCustom = false;
    } catch (e) {
      console.error('Failed to save theme:', e);
    }
  }

  async function handleDeleteCustomTheme(id: string) {
    try {
      await onDeleteCustomTheme(id);
      if (activeTheme === id) {
        onSelectTheme('gruvbox-dark');
      }
    } catch (e) {
      console.error('Failed to delete theme:', e);
    }
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 animate-in fade-in select-none"
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-dialog-title"
  >
    <div
      class="w-full max-w-xl rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] shadow-2xl flex flex-col overflow-hidden"
    >
      <!-- Dialog Header -->
      <div class="px-5 py-4 border-b border-[var(--border-subtle)] flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <Palette size={18} class="text-[var(--accent)]" />
          <h2 id="settings-dialog-title" class="text-sm font-bold text-[var(--text-primary)]">
            Settings & Appearance
          </h2>
        </div>
        <button
          type="button"
          onclick={onClose}
          class="p-1 rounded-md text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card-hover)] transition-colors focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
          title="Close dialog"
          aria-label="Close dialog"
        >
          <X size={16} />
        </button>
      </div>

      <!-- Dialog Body -->
      <div class="p-5 space-y-6 overflow-y-auto max-h-[75vh]">
        <!-- Built-in Theme Presets -->
        <section class="space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)]">
              Color Theme Presets
            </span>
            <span class="text-[11px] text-[var(--text-muted)]">Official palettes</span>
          </div>

          <div class="grid grid-cols-2 gap-2.5">
            {#each THEMES as theme}
              {@const isSelected = activeTheme === theme.id}
              <button
                type="button"
                onclick={() => onSelectTheme(theme.id)}
                class="p-3 rounded-lg border text-left flex items-center justify-between transition-all outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] {isSelected ? 'border-[var(--accent)] bg-[var(--bg-card-active)] ring-1 ring-[var(--accent)]' : 'border-[var(--border-subtle)] bg-[var(--bg-base)] hover:border-[var(--border-strong)] hover:bg-[var(--bg-card-hover)]'}"
              >
                <div class="flex items-center gap-2.5 min-w-0">
                  <div class="flex items-center -space-x-1 shrink-0">
                    <span
                      class="w-4 h-4 rounded-full border border-black/20"
                      style="background-color: {theme.previewBg};"
                    ></span>
                    <span
                      class="w-4 h-4 rounded-full border border-black/20"
                      style="background-color: {theme.previewAccent};"
                    ></span>
                  </div>
                  <span class="text-xs font-medium text-[var(--text-primary)] truncate">
                    {theme.name}
                  </span>
                </div>

                {#if isSelected}
                  <Check size={14} class="text-[var(--accent)] shrink-0 ml-1" />
                {/if}
              </button>
            {/each}
          </div>
        </section>

        <!-- Custom Themes Section (Config File & GUI) -->
        <section class="space-y-3 pt-4 border-t border-[var(--border-subtle)]">
          <div class="flex items-center justify-between">
            <div>
              <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)] block">
                Custom Themes
              </span>
              <span class="text-[11px] text-[var(--text-muted)]">
                Stored in <code class="font-mono text-[10px]">~/.config/kfont/config.yaml</code>
              </span>
            </div>

            <div class="flex items-center gap-2">
              <button
                type="button"
                onclick={handleOpenConfigFile}
                class="px-2.5 py-1 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] text-xs flex items-center gap-1.5 transition-colors cursor-pointer"
                title="Open configuration file in file manager"
              >
                <Folder size={13} />
                <span>Open config.yaml</span>
              </button>

              <button
                type="button"
                onclick={() => (isCreatingCustom = !isCreatingCustom)}
                class="px-2.5 py-1 rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold text-xs flex items-center gap-1 hover:opacity-90 transition-opacity cursor-pointer"
              >
                <Plus size={14} />
                <span>New Theme</span>
              </button>
            </div>
          </div>

          <!-- Existing Custom Themes Grid -->
          {#if customThemes.length > 0}
            <div class="grid grid-cols-2 gap-2.5">
              {#each customThemes as theme}
                {@const isSelected = activeTheme === theme.id}
                <div
                  class="p-3 rounded-lg border text-left flex items-center justify-between transition-all {isSelected ? 'border-[var(--accent)] bg-[var(--bg-card-active)] ring-1 ring-[var(--accent)]' : 'border-[var(--border-subtle)] bg-[var(--bg-base)]'}"
                >
                  <button
                    type="button"
                    onclick={() => onSelectTheme(theme.id, theme)}
                    class="flex-1 flex items-center gap-2.5 min-w-0 text-left outline-none cursor-pointer"
                  >
                    <div class="flex items-center -space-x-1 shrink-0">
                      <span
                        class="w-4 h-4 rounded-full border border-black/20"
                        style="background-color: {theme.bg_base};"
                      ></span>
                      <span
                        class="w-4 h-4 rounded-full border border-black/20"
                        style="background-color: {theme.accent};"
                      ></span>
                    </div>
                    <span class="text-xs font-medium text-[var(--text-primary)] truncate">
                      {theme.name}
                    </span>
                  </button>

                  <div class="flex items-center gap-1 shrink-0 ml-1">
                    {#if isSelected}
                      <Check size={14} class="text-[var(--accent)] mr-1" />
                    {/if}
                    <button
                      type="button"
                      onclick={() => handleDeleteCustomTheme(theme.id)}
                      class="p-1 rounded text-[var(--text-muted)] hover:text-red-400 hover:bg-[var(--bg-card)] transition-colors cursor-pointer"
                      title="Delete theme"
                      aria-label="Delete theme"
                    >
                      <Trash2 size={13} />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}

          <!-- New Theme Creator GUI Form -->
          {#if isCreatingCustom}
            <div class="p-4 rounded-lg bg-[var(--bg-base)] border border-[var(--border-strong)] space-y-4 animate-in fade-in">
              <span class="text-xs font-bold text-[var(--text-primary)] block">Create Theme in GUI</span>

              <div class="grid grid-cols-2 gap-3 text-xs">
                <!-- Theme Name -->
                <div class="col-span-2 space-y-1">
                  <label for="theme-name-input" class="text-[var(--text-muted)] block">Theme Name:</label>
                  <input
                    id="theme-name-input"
                    type="text"
                    bind:value={newThemeName}
                    placeholder="e.g. Cyberpunk 2077"
                    class="w-full px-2.5 py-1.5 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none"
                  />
                </div>

                <!-- Base BG Color -->
                <div class="space-y-1">
                  <span class="text-[var(--text-muted)] block">Background Base:</span>
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={newBgBase}
                      aria-label="Background Base Color"
                      class="w-7 h-7 rounded border-none cursor-pointer bg-transparent"
                    />
                    <input
                      type="text"
                      bind:value={newBgBase}
                      aria-label="Background Base Hex"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] font-mono text-[11px]"
                    />
                  </div>
                </div>

                <!-- Sidebar BG Color -->
                <div class="space-y-1">
                  <span class="text-[var(--text-muted)] block">Sidebar BG:</span>
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={newBgSidebar}
                      aria-label="Sidebar Background Color"
                      class="w-7 h-7 rounded border-none cursor-pointer bg-transparent"
                    />
                    <input
                      type="text"
                      bind:value={newBgSidebar}
                      aria-label="Sidebar Background Hex"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] font-mono text-[11px]"
                    />
                  </div>
                </div>

                <!-- Card BG Color -->
                <div class="space-y-1">
                  <span class="text-[var(--text-muted)] block">Card BG:</span>
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={newBgCard}
                      aria-label="Card Background Color"
                      class="w-7 h-7 rounded border-none cursor-pointer bg-transparent"
                    />
                    <input
                      type="text"
                      bind:value={newBgCard}
                      aria-label="Card Background Hex"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] font-mono text-[11px]"
                    />
                  </div>
                </div>

                <!-- Text Color -->
                <div class="space-y-1">
                  <span class="text-[var(--text-muted)] block">Text Color:</span>
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={newTextPrimary}
                      aria-label="Text Color Picker"
                      class="w-7 h-7 rounded border-none cursor-pointer bg-transparent"
                    />
                    <input
                      type="text"
                      bind:value={newTextPrimary}
                      aria-label="Text Color Hex"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] font-mono text-[11px]"
                    />
                  </div>
                </div>

                <!-- Accent Color -->
                <div class="col-span-2 space-y-1">
                  <span class="text-[var(--text-muted)] block">Accent Color:</span>
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={newAccent}
                      aria-label="Accent Color Picker"
                      class="w-7 h-7 rounded border-none cursor-pointer bg-transparent"
                    />
                    <input
                      type="text"
                      bind:value={newAccent}
                      aria-label="Accent Color Hex"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] font-mono text-[11px]"
                    />
                  </div>
                </div>
              </div>

              <!-- Live Swatch Preview -->
              <div
                class="p-3 rounded-md border flex items-center justify-between text-xs"
                style="background-color: {newBgCard}; border-color: {newBgBase}; color: {newTextPrimary};"
              >
                <span>Theme Live Swatch</span>
                <span class="px-2 py-0.5 rounded font-semibold text-[11px]" style="background-color: {newAccent}; color: {newBgBase};">
                  Accent Action
                </span>
              </div>

              <!-- Buttons -->
              <div class="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  onclick={() => (isCreatingCustom = false)}
                  class="px-3 py-1.5 rounded bg-[var(--bg-card)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)] text-xs cursor-pointer"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onclick={handleSaveNewCustomTheme}
                  class="px-4 py-1.5 rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold text-xs hover:opacity-90 cursor-pointer"
                >
                  Save Theme
                </button>
              </div>
            </div>
          {/if}
        </section>

        <!-- Default Sample Text -->
        <section class="space-y-2 pt-4 border-t border-[var(--border-subtle)]">
          <label for="default-sample-input" class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)] block">
            Default Sample Text
          </label>
          <div class="flex items-center gap-2">
            <input
              id="default-sample-input"
              type="text"
              bind:value={tempDefaultText}
              class="flex-1 px-3 py-1.5 text-xs rounded-md bg-[var(--bg-base)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none"
            />
            <button
              type="button"
              onclick={handleSaveText}
              class="px-3 py-1.5 rounded-md bg-[var(--accent)] text-[var(--accent-text)] font-semibold text-xs hover:opacity-90 transition-opacity cursor-pointer"
            >
              Save
            </button>
          </div>
        </section>
      </div>

      <!-- Dialog Footer -->
      <div class="px-5 py-3 border-t border-[var(--border-subtle)] bg-[var(--bg-sidebar)] flex justify-end">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-1.5 rounded-md bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] text-[var(--text-primary)] border border-[var(--border-subtle)] text-xs font-medium transition-colors cursor-pointer"
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}
