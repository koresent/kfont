<script lang="ts">
  import { Copy, Check, Search, ChevronLeft, ChevronRight } from '@lucide/svelte';
  import type { FontSummary, FontDetails } from '$lib/types';

  let {
    font,
    details,
    fontFamilyCss,
  }: {
    font: FontSummary;
    details: FontDetails | null;
    fontFamilyCss: string;
  } = $props();

  let searchQuery = $state('');
  let selectedCategory = $state('all');
  let selectedCodepoint = $state<number | null>(65); // Default to 'A'
  let copiedFeedback = $state(false);
  let currentPage = $state(1);
  const pageSize = 120;

  const categories = [
    { id: 'all', label: 'All Glyphs', min: 0, max: 0x10ffff },
    { id: 'latin', label: 'Basic Latin', min: 0x0020, max: 0x007e },
    { id: 'latin-ext', label: 'Latin Supplement', min: 0x0080, max: 0x024f },
    { id: 'cyrillic', label: 'Cyrillic', min: 0x0400, max: 0x04ff },
    { id: 'numbers', label: 'Numbers & Math', min: 0x0030, max: 0x22ff },
    { id: 'symbols', label: 'Punctuation & Symbols', min: 0x2000, max: 0x2bff },
  ];

  let allCodepoints = $derived(details?.codepoints || []);

  let filteredCodepoints = $derived.by(() => {
    let list = allCodepoints;
    const cat = categories.find((c) => c.id === selectedCategory);
    if (cat && cat.id !== 'all') {
      list = list.filter((cp) => cp >= cat.min && cp <= cat.max);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.trim().toLowerCase();
      list = list.filter((cp) => {
        const hex = cp.toString(16).toLowerCase();
        const char = String.fromCodePoint(cp).toLowerCase();
        return hex.includes(q) || char.includes(q);
      });
    }
    return list;
  });

  // Reset page when category or search changes
  $effect(() => {
    selectedCategory;
    searchQuery;
    currentPage = 1;
  });

  let totalPages = $derived(Math.max(1, Math.ceil(filteredCodepoints.length / pageSize)));

  let pageCodepoints = $derived(
    filteredCodepoints.slice((currentPage - 1) * pageSize, currentPage * pageSize)
  );

  // Selected glyph properties
  let selectedChar = $derived(selectedCodepoint !== null ? String.fromCodePoint(selectedCodepoint) : '');
  let selectedHex = $derived(selectedCodepoint !== null ? `U+${selectedCodepoint.toString(16).toUpperCase().padStart(4, '0')}` : '');
  let selectedDec = $derived(selectedCodepoint !== null ? selectedCodepoint.toString() : '');
  let selectedHtml = $derived(selectedCodepoint !== null ? `&#${selectedCodepoint};` : '');

  async function copyToClipboard(text: string) {
    try {
      await navigator.clipboard.writeText(text);
      copiedFeedback = true;
      setTimeout(() => (copiedFeedback = false), 2000);
    } catch (e) {
      console.error('Failed to copy', e);
    }
  }
</script>

<div class="h-full flex flex-col overflow-hidden">
  <!-- Top Filters & Search Bar -->
  <div class="p-4 border-b border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-wrap items-center justify-between gap-3 select-none">
    <!-- Category pills -->
    <div class="flex flex-wrap items-center gap-1.5 text-xs">
      {#each categories as cat}
        <button
          type="button"
          onclick={() => (selectedCategory = cat.id)}
          class="px-2.5 py-1 rounded transition-colors {selectedCategory === cat.id ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'bg-[var(--bg-base)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
        >
          {cat.label}
        </button>
      {/each}
    </div>

    <!-- Search & Pagination Row -->
    <div class="flex items-center gap-3">
      <div class="relative flex items-center">
        <Search size={14} class="absolute left-2.5 text-[var(--text-muted)] pointer-events-none" />
        <input
          type="text"
          placeholder="Hex (e.g. 0416) or char..."
          bind:value={searchQuery}
          class="pl-8 pr-3 py-1 text-xs rounded bg-[var(--bg-base)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none w-48"
          aria-label="Search glyphs"
        />
      </div>

      <!-- Pagination Controls -->
      {#if totalPages > 1}
        <div class="flex items-center gap-1.5 text-xs text-[var(--text-muted)]">
          <button
            type="button"
            disabled={currentPage <= 1}
            onclick={() => (currentPage = Math.max(1, currentPage - 1))}
            class="p-1 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] disabled:opacity-30 hover:bg-[var(--bg-card-hover)]"
            title="Previous page"
            aria-label="Previous page"
          >
            <ChevronLeft size={14} />
          </button>

          <span class="font-mono text-[11px] tabular-nums px-1">
            {currentPage} / {totalPages}
          </span>

          <button
            type="button"
            disabled={currentPage >= totalPages}
            onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
            class="p-1 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] disabled:opacity-30 hover:bg-[var(--bg-card-hover)]"
            title="Next page"
            aria-label="Next page"
          >
            <ChevronRight size={14} />
          </button>
        </div>
      {/if}
    </div>
  </div>

  <!-- Content Split: Glyphs Grid + Inspector Sidebar -->
  <div class="flex-1 flex overflow-hidden">
    <!-- Glyphs Grid (Fast Chunked Rendering) -->
    <div class="flex-1 overflow-y-auto p-4 flex flex-col justify-between">
      <div class="grid grid-cols-6 sm:grid-cols-8 md:grid-cols-10 lg:grid-cols-12 gap-1.5">
        {#each pageCodepoints as cp (cp)}
          {@const char = String.fromCodePoint(cp)}
          {@const isSelected = selectedCodepoint === cp}
          <button
            type="button"
            onclick={() => (selectedCodepoint = cp)}
            class="h-12 flex flex-col items-center justify-center rounded border transition-colors relative group focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none {isSelected ? 'bg-[var(--bg-card-active)] border-[var(--accent)] text-[var(--accent)] font-bold' : 'bg-[var(--bg-card)] border-[var(--border-subtle)] text-[var(--text-primary)] hover:border-[var(--border-strong)] hover:bg-[var(--bg-card-hover)]'}"
            title={`U+${cp.toString(16).toUpperCase().padStart(4, '0')}: ${char}`}
          >
            <span class="text-xl leading-none" style="font-family: {fontFamilyCss};">
              {char}
            </span>
            <span class="text-[9px] text-[var(--text-muted)] font-mono leading-none mt-1 opacity-70 group-hover:opacity-100">
              {cp.toString(16).toUpperCase().padStart(4, '0')}
            </span>
          </button>
        {/each}
      </div>

      {#if filteredCodepoints.length === 0}
        <div class="p-12 text-center text-xs text-[var(--text-muted)]">
          No glyphs match the current filter or search criteria.
        </div>
      {:else}
        <!-- Bottom Pagination Bar -->
        <div class="pt-4 border-t border-[var(--border-subtle)] mt-4 flex items-center justify-between text-xs text-[var(--text-muted)] select-none">
          <span>
            Showing {(currentPage - 1) * pageSize + 1} to {Math.min(currentPage * pageSize, filteredCodepoints.length)} of {filteredCodepoints.length.toLocaleString()} glyphs
          </span>

          {#if totalPages > 1}
            <div class="flex items-center gap-2">
              <button
                type="button"
                disabled={currentPage <= 1}
                onclick={() => (currentPage = Math.max(1, currentPage - 1))}
                class="px-2.5 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] disabled:opacity-30 hover:bg-[var(--bg-card-hover)]"
              >
                Previous
              </button>
              <button
                type="button"
                disabled={currentPage >= totalPages}
                onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
                class="px-2.5 py-1 rounded bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] disabled:opacity-30 hover:bg-[var(--bg-card-hover)]"
              >
                Next
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Inspector Sidebar -->
    <aside class="w-72 border-l border-[var(--border-subtle)] bg-[var(--bg-card)] p-5 flex flex-col justify-between overflow-y-auto select-none shrink-0" aria-label="Glyph Inspector">
      <div class="space-y-6">
        <span class="text-[11px] font-semibold uppercase tracking-wider text-[var(--text-muted)] block">
          Glyph Inspector
        </span>

        <!-- Giant Character Preview -->
        <div class="h-32 rounded-lg bg-[var(--bg-base)] border border-[var(--border-subtle)] flex items-center justify-center overflow-hidden">
          <span class="text-7xl text-[var(--text-primary)]" style="font-family: {fontFamilyCss};">
            {selectedChar}
          </span>
        </div>

        <!-- Details List -->
        <div class="space-y-3 text-xs">
          <div class="flex items-center justify-between border-b border-[var(--border-subtle)] pb-2">
            <span class="text-[var(--text-muted)]">Unicode Point:</span>
            <span class="font-mono font-semibold text-[var(--text-primary)]">{selectedHex}</span>
          </div>

          <div class="flex items-center justify-between border-b border-[var(--border-subtle)] pb-2">
            <span class="text-[var(--text-muted)]">Decimal:</span>
            <span class="font-mono text-[var(--text-primary)]">{selectedDec}</span>
          </div>

          <div class="flex items-center justify-between border-b border-[var(--border-subtle)] pb-2">
            <span class="text-[var(--text-muted)]">HTML Entity:</span>
            <span class="font-mono text-[var(--text-primary)]">{selectedHtml}</span>
          </div>

          <div class="flex items-center justify-between border-b border-[var(--border-subtle)] pb-2">
            <span class="text-[var(--text-muted)]">Total Glyphs:</span>
            <span class="font-mono text-[var(--text-primary)]">{allCodepoints.length.toLocaleString()}</span>
          </div>
        </div>
      </div>

      <!-- Copy Action Buttons -->
      <div class="space-y-2 pt-6 border-t border-[var(--border-subtle)]">
        <button
          type="button"
          onclick={() => copyToClipboard(selectedChar)}
          class="w-full py-2 px-3 rounded-md bg-[var(--accent)] text-[var(--accent-text)] font-semibold text-xs flex items-center justify-center gap-2 hover:opacity-90 transition-opacity focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
        >
          {#if copiedFeedback}
            <Check size={15} />
            <span>Copied to Clipboard!</span>
          {:else}
            <Copy size={15} />
            <span>Copy Character ({selectedChar})</span>
          {/if}
        </button>

        <button
          type="button"
          onclick={() => copyToClipboard(selectedHex)}
          class="w-full py-1.5 px-3 rounded-md bg-[var(--bg-base)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] border border-[var(--border-subtle)] text-xs flex items-center justify-center gap-2 transition-colors focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
        >
          <span>Copy Hex Code ({selectedHex})</span>
        </button>
      </div>
    </aside>
  </div>
</div>
