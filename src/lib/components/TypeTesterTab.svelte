<script lang="ts">
  import { RotateCcw, Contrast } from '@lucide/svelte';
  import type { FontSummary } from '$lib/types';

  let {
    font,
    fontFamilyCss,
  }: {
    font: FontSummary;
    fontFamilyCss: string;
  } = $props();

  function getDefaultText(fam: string) {
    return `Typography is the craft of endowing human language with a durable visual form.

When you test a typeface, look for rhythm and spacing between letters. Good type feels harmonious both in large headline sizes and dense text blocks.

Try typing your own sentences, numbers (0123456789), and punctuation (!?@#$%) here to see how they render in ${fam}.`;
  }

  let text = $state('');

  $effect(() => {
    if (!text) {
      text = getDefaultText(font.family);
    }
  });
  let fontSize = $state(28);
  let lineHeight = $state(1.5);
  let letterSpacing = $state(0);
  let isInverted = $state(false);

  let wordCount = $derived(text.trim() ? text.trim().split(/\s+/).length : 0);
  let charCount = $derived(text.length);
</script>

<div class="h-full flex flex-col overflow-hidden">
  <!-- Controls Toolbar -->
  <div class="p-4 border-b border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-wrap items-center justify-between gap-4 select-none">
    <div class="flex flex-wrap items-center gap-6 text-xs">
      <!-- Font Size -->
      <div class="flex items-center gap-2">
        <span class="text-[var(--text-muted)]">Size:</span>
        <input
          type="range"
          min="14"
          max="120"
          bind:value={fontSize}
          class="w-24 accent-[var(--accent)] cursor-pointer"
        />
        <span class="font-mono w-10 text-[var(--text-primary)]">{fontSize}px</span>
      </div>

      <!-- Line Height -->
      <div class="flex items-center gap-2">
        <span class="text-[var(--text-muted)]">Leading:</span>
        <input
          type="range"
          min="1.0"
          max="2.5"
          step="0.05"
          bind:value={lineHeight}
          class="w-20 accent-[var(--accent)] cursor-pointer"
        />
        <span class="font-mono w-8 text-[var(--text-primary)]">{lineHeight.toFixed(2)}</span>
      </div>

      <!-- Letter Spacing -->
      <div class="flex items-center gap-2">
        <span class="text-[var(--text-muted)]">Tracking:</span>
        <input
          type="range"
          min="-2"
          max="10"
          step="0.5"
          bind:value={letterSpacing}
          class="w-20 accent-[var(--accent)] cursor-pointer"
        />
        <span class="font-mono w-8 text-[var(--text-primary)]">{letterSpacing}px</span>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2 text-xs">
      <button
        type="button"
        onclick={() => (isInverted = !isInverted)}
        class="px-2.5 py-1 rounded border border-[var(--border-subtle)] flex items-center gap-1.5 transition-colors {isInverted ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'bg-[var(--bg-base)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
        title="Invert typing colors"
      >
        <Contrast size={14} />
        <span>Invert</span>
      </button>

      <button
        type="button"
        onclick={() => (text = getDefaultText(font.family))}
        class="px-2.5 py-1 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card-hover)] flex items-center gap-1.5 transition-colors"
        title="Reset text"
      >
        <RotateCcw size={14} />
        <span>Reset</span>
      </button>
    </div>
  </div>

  <!-- Big Interactive Typing Canvas -->
  <div class="flex-1 p-6 overflow-hidden flex flex-col {isInverted ? 'bg-[var(--text-primary)] text-[var(--bg-base)]' : 'bg-[var(--bg-base)] text-[var(--text-primary)]'}">
    <textarea
      bind:value={text}
      placeholder="Type anything here..."
      class="flex-1 w-full bg-transparent resize-none outline-none border-none leading-relaxed transition-all p-2"
      style="
        font-family: {fontFamilyCss};
        font-size: {fontSize}px;
        line-height: {lineHeight};
        letter-spacing: {letterSpacing}px;
        font-weight: {font.weight};
        font-style: {font.is_italic ? 'italic' : 'normal'};
      "
      aria-label="Font test sandbox input"
    ></textarea>

    <!-- Bottom Counter -->
    <div class="pt-3 border-t border-[var(--border-subtle)] flex items-center justify-between text-xs opacity-70 select-none">
      <span>{font.full_name}</span>
      <div class="flex items-center gap-4 font-mono">
        <span>{wordCount} words</span>
        <span>{charCount} characters</span>
      </div>
    </div>
  </div>
</div>
