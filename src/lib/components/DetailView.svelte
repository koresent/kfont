<script lang="ts">
  import { Type, Grid, Info, Edit3, Layers, Sliders, Code } from '@lucide/svelte';
  import type { FontSummary, FontDetails } from '$lib/types';
  import SpecimenTab from './SpecimenTab.svelte';
  import WaterfallTab from './WaterfallTab.svelte';
  import GlyphsTab from './GlyphsTab.svelte';
  import LigaturesTab from './LigaturesTab.svelte';
  import OpenTypeTab from './OpenTypeTab.svelte';
  import PropertiesTab from './PropertiesTab.svelte';
  import TypeTesterTab from './TypeTesterTab.svelte';
  import { ensureFontLoaded } from '$lib/fontLoader';

  let {
    font,
    details,
    familyStyles = [],
    isLoadingDetails = false,
    onSelectStyle,
  }: {
    font: FontSummary | null;
    details: FontDetails | null;
    familyStyles: FontSummary[];
    isLoadingDetails: boolean;
    onSelectStyle: (style: FontSummary) => void;
  } = $props();

  let activeTab = $state<'specimen' | 'waterfall' | 'glyphs' | 'ligatures' | 'opentype' | 'properties' | 'tester'>('specimen');
  let currentLoadedFamily = $state('sans-serif');

  $effect(() => {
    if (font) {
      ensureFontLoaded(font.file_path, font.id).then((fam) => {
        currentLoadedFamily = fam;
      });
    }
  });

  const tabs = [
    { id: 'specimen', label: 'Specimen', icon: Type },
    { id: 'waterfall', label: 'Waterfall', icon: Layers },
    { id: 'glyphs', label: 'Glyphs', icon: Grid },
    { id: 'ligatures', label: 'Ligatures', icon: Code },
    { id: 'opentype', label: 'OpenType', icon: Sliders },
    { id: 'properties', label: 'Info & Metrics', icon: Info },
    { id: 'tester', label: 'Type Tester', icon: Edit3 },
  ];
</script>

<main class="flex-1 flex flex-col h-full bg-[var(--bg-base)] overflow-hidden" aria-label="Font Details and Preview">
  {#if !font}
    <div class="flex-1 flex flex-col items-center justify-center p-8 text-center text-[var(--text-muted)] gap-3 select-none">
      <div class="w-12 h-12 rounded-xl bg-[var(--bg-card)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--accent)]">
        <Type size={24} />
      </div>
      <div>
        <h2 class="text-sm font-semibold text-[var(--text-primary)]">No font selected</h2>
        <p class="text-xs text-[var(--text-muted)] mt-1">Choose a font from the list on the left to preview specimen, glyphs, and metrics</p>
      </div>
    </div>
  {:else}
    <!-- Top Header Bar -->
    <header class="p-4 border-b border-[var(--border-subtle)] bg-[var(--bg-card)] space-y-3 shrink-0 select-none">
      <div class="flex flex-wrap items-center justify-between gap-4">
        <!-- Family and Styles -->
        <div class="space-y-1 min-w-0">
          <div class="flex items-center gap-3">
            <h1 class="text-xl font-bold text-[var(--text-primary)] tracking-tight truncate" title={font.family}>
              {font.family}
            </h1>
            <span class="px-2 py-0.5 rounded text-[10px] font-semibold uppercase bg-[var(--badge-bg)] text-[var(--badge-text)] tracking-wider shrink-0">
              {font.format}
            </span>
          </div>

          <!-- Style Buttons / Selector -->
          {#if familyStyles.length > 1}
            <div class="flex flex-wrap items-center gap-1.5 pt-1">
              {#each familyStyles as styleItem}
                <button
                  type="button"
                  onclick={() => onSelectStyle(styleItem)}
                  class="px-2.5 py-0.5 rounded text-xs transition-colors {font.id === styleItem.id ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold shadow-xs' : 'bg-[var(--bg-base)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
                >
                  {styleItem.style}
                </button>
              {/each}
            </div>
          {:else}
            <p class="text-xs text-[var(--text-muted)]">{font.style}</p>
          {/if}
        </div>

        <!-- Badges & Stats -->
        <div class="flex items-center gap-3 text-xs text-[var(--text-muted)]">
          <div class="flex flex-col items-end">
            <span class="text-[var(--text-primary)] font-mono font-medium">{font.glyph_count.toLocaleString()} glyphs</span>
            <span class="text-[11px]">{font.file_size_formatted}</span>
          </div>
        </div>
      </div>

      <!-- Tab Buttons -->
      <nav class="flex items-center gap-1 pt-1 border-t border-[var(--border-subtle)] overflow-x-auto" aria-label="Font Detail Views">
        {#each tabs as tab}
          {@const Icon = tab.icon}
          {@const isActive = activeTab === tab.id}
          <button
            type="button"
            onclick={() => (activeTab = tab.id as any)}
            class="px-3 py-1.5 rounded-md text-xs font-medium flex items-center gap-2 transition-colors outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] shrink-0 {isActive ? 'bg-[var(--bg-card-active)] text-[var(--accent)] font-semibold' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)] hover:text-[var(--text-primary)]'}"
          >
            <Icon size={14} />
            <span>{tab.label}</span>
          </button>
        {/each}
      </nav>
    </header>

    <!-- Tab Content -->
    <div class="flex-1 overflow-hidden relative">
      {#if isLoadingDetails}
        <div class="absolute inset-0 bg-[var(--bg-base)]/60 backdrop-blur-xs flex items-center justify-center z-10">
          <div class="w-6 h-6 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin"></div>
        </div>
      {/if}

      {#if activeTab === 'specimen'}
        <SpecimenTab {font} {details} fontFamilyCss={currentLoadedFamily} />
      {:else if activeTab === 'waterfall'}
        <WaterfallTab {font} fontFamilyCss={currentLoadedFamily} />
      {:else if activeTab === 'glyphs'}
        <GlyphsTab {font} {details} fontFamilyCss={currentLoadedFamily} />
      {:else if activeTab === 'ligatures'}
        <LigaturesTab {font} fontFamilyCss={currentLoadedFamily} />
      {:else if activeTab === 'opentype'}
        <OpenTypeTab {font} fontFamilyCss={currentLoadedFamily} />
      {:else if activeTab === 'properties'}
        <PropertiesTab {font} {details} />
      {:else if activeTab === 'tester'}
        <TypeTesterTab {font} fontFamilyCss={currentLoadedFamily} />
      {/if}
    </div>
  {/if}
</main>
