<script lang="ts">
  import { Search, X } from '@lucide/svelte';
  import type { FontFamilyGroup, FontSummary } from '$lib/types';
  import { ensureFontLoaded, isFontLoaded, getCustomFamilyName } from '$lib/fontLoader';
  import Select from './Select.svelte';

  let {
    groups = [],
    selectedFamily = '',
    selectedFontId = '',
    searchQuery = '',
    previewText = 'The quick brown fox jumps over the lazy dog',
    fontSize = 22,
    sortBy = 'name-asc',
    isLoading = false,
    onSelectGroup,
    onSearchChange,
    onPreviewTextChange,
    onFontSizeChange,
    onSortChange,
  }: {
    groups: FontFamilyGroup[];
    selectedFamily: string;
    selectedFontId: string;
    searchQuery: string;
    previewText: string;
    fontSize: number;
    sortBy: string;
    isLoading: boolean;
    onSelectGroup: (group: FontFamilyGroup) => void;
    onSearchChange: (q: string) => void;
    onPreviewTextChange: (t: string) => void;
    onFontSizeChange: (s: number) => void;
    onSortChange: (sort: string) => void;
  } = $props();

  const ITEM_HEIGHT = 84; // Fixed height per font card for perfect 60fps virtualization
  const OVERSCAN_COUNT = 30; // 30 items above and 30 items below (~2,520px buffer on each side to eliminate blank space)

  let scrollContainer = $state<HTMLElement | null>(null);
  let scrollTop = $state(0);
  let containerHeight = $state(600);
  let rafId: number | null = null;
  let loadedVersion = $state(0);

  function handleScroll(e: Event) {
    const target = e.currentTarget as HTMLElement;
    const currentScrollTop = target.scrollTop;
    if (rafId !== null) return;
    rafId = requestAnimationFrame(() => {
      scrollTop = currentScrollTop;
      rafId = null;
    });
  }

  // Measure container height on resize
  $effect(() => {
    if (scrollContainer) {
      containerHeight = scrollContainer.clientHeight || 600;
      const ro = new ResizeObserver((entries) => {
        for (const entry of entries) {
          containerHeight = entry.contentRect.height;
        }
      });
      ro.observe(scrollContainer);
      return () => {
        ro.disconnect();
        if (rafId !== null) {
          cancelAnimationFrame(rafId);
          rafId = null;
        }
      };
    }
  });

  // Calculate virtual slice with large overscan buffer
  let totalHeight = $derived(groups.length * ITEM_HEIGHT);

  let startIndex = $derived(
    Math.max(0, Math.floor(scrollTop / ITEM_HEIGHT) - OVERSCAN_COUNT)
  );

  let visibleCount = $derived(
    Math.ceil(containerHeight / ITEM_HEIGHT) + (OVERSCAN_COUNT * 2)
  );

  let endIndex = $derived(
    Math.min(groups.length, startIndex + visibleCount)
  );

  let visibleGroups = $derived(groups.slice(startIndex, endIndex));
  let topOffset = $derived(startIndex * ITEM_HEIGHT);

  function getFontFamilyStyle(font: FontSummary, _version: number): string {
    const custom = isFontLoaded(font.id) ? `'${getCustomFamilyName(font.id)}', ` : '';
    const escapedFamily = font.family.replace(/'/g, "\\'");
    const escapedPostscript = font.postscript_name.replace(/'/g, "\\'");
    return `${custom}'${escapedFamily}', '${escapedPostscript}', sans-serif`;
  }

  // Fast, lightweight on-demand loader for the ~12 fonts currently in the user viewport
  let loadDebounceTimer: any = null;
  $effect(() => {
    const st = scrollTop;
    const ch = containerHeight;
    const all = groups;

    clearTimeout(loadDebounceTimer);
    loadDebounceTimer = setTimeout(() => {
      const vStart = Math.max(0, Math.floor(st / ITEM_HEIGHT));
      const vCount = Math.ceil(ch / ITEM_HEIGHT) + 2;
      const vEnd = Math.min(all.length, vStart + vCount);
      const viewportItems = all.slice(vStart, vEnd);

      for (const grp of viewportItems) {
        const f = grp.defaultFont;
        if (!isFontLoaded(f.id)) {
          ensureFontLoaded(f.file_path, f.id).then(() => {
            loadedVersion = loadedVersion + 1;
          });
        }
      }
    }, 60);

    return () => clearTimeout(loadDebounceTimer);
  });

  const sortOptions = [
    { value: 'name-asc', label: 'A to Z' },
    { value: 'name-desc', label: 'Z to A' },
    { value: 'styles-desc', label: 'Most Styles' },
    { value: 'glyphs-desc', label: 'Most Glyphs' },
  ];

  let samplePreset = $state<'fox' | 'cyrillic' | 'alphabet' | 'numbers' | 'name' | 'custom'>('fox');

  const samplePresets = [
    { value: 'fox', label: 'English (Fox)' },
    { value: 'cyrillic', label: 'Русский (Панграмма)' },
    { value: 'alphabet', label: 'Alphabet (Aa-Zz)' },
    { value: 'numbers', label: 'Numbers & Symbols' },
    { value: 'name', label: 'Font Family Name' },
    { value: 'custom', label: 'Custom Text' },
  ];

  function handlePresetChange(preset: string) {
    samplePreset = preset as any;
    if (preset === 'fox') {
      onPreviewTextChange('The quick brown fox jumps over the lazy dog');
    } else if (preset === 'cyrillic') {
      onPreviewTextChange('Съешь же ещё этих мягких французских булок, да выпей чаю');
    } else if (preset === 'alphabet') {
      onPreviewTextChange('ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz');
    } else if (preset === 'numbers') {
      onPreviewTextChange('0123456789 !@#$%^&*()_+{}[]:;\'"<>,.?');
    } else if (preset === 'name') {
      onPreviewTextChange('');
    }
  }
</script>

<div class="flex-1 flex flex-col h-full bg-[var(--bg-base)] overflow-hidden border-r border-[var(--border-subtle)] min-w-[320px] max-w-[480px]">
  <!-- Search & Toolbar -->
  <div class="p-3 border-b border-[var(--border-subtle)] space-y-2.5 bg-[var(--bg-base)] select-none">
    <!-- Search Bar -->
    <div class="relative flex items-center">
      <Search size={15} class="absolute left-3 text-[var(--text-muted)] pointer-events-none" />
      <input
        type="text"
        placeholder="Search fonts by name, style, designer..."
        value={searchQuery}
        oninput={(e) => onSearchChange((e.target as HTMLInputElement).value)}
        class="w-full pl-9 pr-8 py-1.5 text-xs rounded-md bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] placeholder:text-[var(--text-muted)] focus:border-[var(--accent)] focus:ring-1 focus:ring-[var(--accent)] outline-none transition-all"
        aria-label="Search fonts"
      />
      {#if searchQuery}
        <button
          type="button"
          onclick={() => onSearchChange('')}
          class="absolute right-2.5 p-0.5 text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors cursor-pointer"
          title="Clear search"
          aria-label="Clear search"
        >
          <X size={14} />
        </button>
      {/if}
    </div>

    <!-- Quick Preview Input & Preset Bar -->
    <div class="flex items-center gap-2 text-xs">
      <!-- Preset Dropdown -->
      <div class="shrink-0">
        <Select
          value={samplePreset}
          options={samplePresets}
          align="left"
          ariaLabel="Sample text preset"
          onChange={handlePresetChange}
        />
      </div>

      <!-- Preview text input -->
      <div class="flex-1 min-w-0">
        <input
          type="text"
          value={samplePreset === 'name' ? 'Displaying font names' : previewText}
          disabled={samplePreset === 'name'}
          oninput={(e) => {
            samplePreset = 'custom';
            onPreviewTextChange((e.target as HTMLInputElement).value);
          }}
          placeholder="Type custom preview text..."
          class="w-full px-2.5 py-1 text-xs rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] placeholder:text-[var(--text-muted)] focus:border-[var(--accent)] outline-none truncate disabled:opacity-60 disabled:cursor-not-allowed"
          title="Custom list preview text"
          aria-label="Custom list preview text"
        />
      </div>
    </div>

    <!-- Controls Row: Size slider & Sort -->
    <div class="flex items-center justify-between gap-2 text-xs pt-0.5">
      <!-- Size slider -->
      <div class="flex items-center gap-2" title={`Preview size: ${fontSize}px`}>
        <span class="text-[11px] text-[var(--text-muted)] font-medium">Size:</span>
        <input
          type="range"
          min="14"
          max="44"
          step="2"
          value={fontSize}
          oninput={(e) => onFontSizeChange(Number((e.target as HTMLInputElement).value))}
          class="w-20 accent-[var(--accent)] cursor-pointer"
          aria-label="List preview font size"
        />
        <span class="text-[10px] text-[var(--text-muted)] tabular-nums w-4">{fontSize}</span>
      </div>

      <!-- Custom Themed Sort Dropdown (No white background) -->
      <div class="flex items-center gap-1.5 shrink-0">
        <span class="text-[11px] text-[var(--text-muted)] font-medium">Sort:</span>
        <Select
          value={sortBy}
          options={sortOptions}
          align="right"
          ariaLabel="Sort order"
          onChange={(v) => onSortChange(v)}
        />
      </div>
    </div>
  </div>

  <!-- Virtualized 60 FPS Fonts List View -->
  <div
    bind:this={scrollContainer}
    onscroll={handleScroll}
    class="flex-1 overflow-y-auto relative contain-strict"
    role="listbox"
    aria-label="Font List"
  >
    {#if isLoading}
      <div class="p-8 text-center text-xs text-[var(--text-muted)] flex flex-col items-center justify-center gap-3">
        <div class="w-6 h-6 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
        <span>Scanning fonts...</span>
      </div>
    {:else if groups.length === 0}
      <div class="p-12 text-center text-xs text-[var(--text-muted)] flex flex-col items-center justify-center gap-2">
        <p class="font-medium text-[var(--text-primary)]">No fonts found</p>
        <p class="text-[11px]">Try adjusting your search query or folder selection</p>
        {#if searchQuery}
          <button
            type="button"
            onclick={() => onSearchChange('')}
            class="mt-2 px-3 py-1 rounded bg-[var(--bg-card)] text-[var(--accent)] hover:bg-[var(--bg-card-hover)] transition-colors text-xs font-medium cursor-pointer"
          >
            Clear Search
          </button>
        {/if}
      </div>
    {:else}
      <!-- Invisible spacer with total virtual height -->
      <div style="height: {totalHeight}px; width: 100%; position: relative;">
        <!-- Container shifted to top offset holding only ~70 DOM cards -->
        <div
          class="divide-y divide-[var(--border-subtle)]"
          style="transform: translateY({topOffset}px); will-change: transform; position: absolute; left: 0; right: 0; top: 0;"
        >
          {#each visibleGroups as grp, idx (grp.family)}
            {@const font = grp.defaultFont}
            {@const isSelected = selectedFamily.toLowerCase() === grp.family.toLowerCase()}
            {@const displayText = samplePreset === 'name' ? grp.family : (previewText || grp.family)}
            <div
              role="option"
              aria-selected={isSelected}
              tabindex="0"
              onclick={() => onSelectGroup(grp)}
              onkeydown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  onSelectGroup(grp);
                }
              }}
              style="height: {ITEM_HEIGHT}px;"
              class="p-3 cursor-pointer transition-colors text-left outline-none flex flex-col justify-between focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[var(--accent)] {isSelected ? 'bg-[var(--bg-card-active)] border-l-4 border-l-[var(--accent)]' : 'hover:bg-[var(--bg-card-hover)]'}"
            >
              <!-- Metadata Header Row -->
              <div class="flex items-center justify-between gap-2 select-none">
                <div class="flex items-center gap-2 truncate">
                  <span class="text-xs font-bold text-[var(--text-primary)] truncate" title={grp.family}>
                    {grp.family}
                  </span>
                  <span class="text-[10px] text-[var(--text-muted)] font-normal shrink-0">
                    {grp.styles.length} {grp.styles.length === 1 ? 'style' : 'styles'}
                  </span>
                </div>

                <div class="flex items-center gap-1.5 shrink-0">
                  {#each grp.formats as fmt}
                    <span class="px-1.5 py-0.5 rounded text-[9px] font-semibold uppercase bg-[var(--badge-bg)] text-[var(--badge-text)] tracking-wider">
                      {fmt}
                    </span>
                  {/each}
                  {#if grp.isMonospaced}
                    <span class="px-1 py-0.5 rounded text-[9px] font-mono bg-[var(--bg-card)] text-[var(--text-muted)]">
                      mono
                    </span>
                  {/if}
                </div>
              </div>

              <!-- Live Font Rendering Line with Real Typeface -->
              <div
                class="overflow-hidden whitespace-nowrap text-ellipsis text-[var(--text-primary)] leading-normal select-text"
                style="font-family: {getFontFamilyStyle(font, loadedVersion)}; font-size: {fontSize}px; font-weight: {font.weight}; font-style: {font.is_italic ? 'italic' : 'normal'};"
                title={displayText}
              >
                {displayText}
              </div>

              <!-- Sub info line -->
              <div class="flex items-center justify-between text-[10px] text-[var(--text-muted)] select-none">
                <span class="truncate max-w-[200px]">{font.file_name}</span>
                <span>{grp.glyphCount.toLocaleString()} glyphs</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
