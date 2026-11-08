<script lang="ts">
  import type { FontSummary } from '$lib/types';

  let {
    font,
    fontFamilyCss,
  }: {
    font: FontSummary;
    fontFamilyCss: string;
  } = $props();

  const PANGRAMS = [
    'The quick brown fox jumps over the lazy dog.',
    'Съешь же ещё этих мягких французских булок, да выпей чаю.',
    'Grumpy wizards make toxic brew for the evil Queen and Jack.',
    '0123456789 !@#$%^&*()_+-=[]{}|;:,.<>?',
  ];

  let sampleText = $state('The quick brown fox jumps over the lazy dog.');
  const sizes = [10, 12, 14, 16, 18, 21, 24, 28, 32, 36, 42, 48, 54, 60, 72, 84, 96];
</script>

<div class="h-full flex flex-col overflow-hidden">
  <!-- Top input toolbar -->
  <div class="p-4 border-b border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-wrap items-center justify-between gap-3 select-none">
    <div class="flex-1 min-w-[280px]">
      <input
        type="text"
        bind:value={sampleText}
        placeholder="Type sample text for waterfall..."
        class="w-full px-3 py-1.5 text-xs rounded bg-[var(--bg-base)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none"
      />
    </div>

    <!-- Quick Pangram Presets -->
    <div class="flex items-center gap-1 text-xs">
      {#each PANGRAMS as p, idx}
        <button
          type="button"
          onclick={() => (sampleText = p)}
          class="px-2 py-1 rounded bg-[var(--bg-base)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] transition-colors truncate max-w-[120px]"
          title={p}
        >
          {idx === 0 ? 'Fox (EN)' : idx === 1 ? 'Булки (RU)' : idx === 2 ? 'Wizards' : 'Numbers'}
        </button>
      {/each}
    </div>
  </div>

  <!-- Waterfall List -->
  <div class="flex-1 overflow-y-auto p-6 space-y-5 select-text">
    {#each sizes as size}
      <div class="flex items-baseline gap-5 group border-b border-[var(--border-subtle)]/50 pb-3">
        <span class="w-14 text-right font-mono text-xs text-[var(--text-muted)] shrink-0 select-none group-hover:text-[var(--accent)] transition-colors">
          {size} pt
        </span>
        <div
          class="flex-1 overflow-hidden whitespace-nowrap text-ellipsis text-[var(--text-primary)] leading-tight"
          style="
            font-family: {fontFamilyCss};
            font-size: {size}px;
            font-weight: {font.weight};
            font-style: {font.is_italic ? 'italic' : 'normal'};
          "
        >
          {sampleText || font.family}
        </div>
      </div>
    {/each}
  </div>
</div>
