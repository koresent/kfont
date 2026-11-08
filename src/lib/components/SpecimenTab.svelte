<script lang="ts">
  import { AlignLeft, AlignCenter, AlignRight, Sliders, Type } from '@lucide/svelte';
  import type { FontSummary, FontDetails } from '$lib/types';
  import Select from './Select.svelte';

  let {
    font,
    details,
    fontFamilyCss,
  }: {
    font: FontSummary;
    details: FontDetails | null;
    fontFamilyCss: string;
  } = $props();

  const PANGRAMS = [
    { label: 'English (Fox)', text: 'The quick brown fox jumps over the lazy dog.' },
    { label: 'English (Wizards)', text: 'Grumpy wizards make toxic brew for the evil Queen and Jack.' },
    { label: 'Russian (Булки)', text: 'Съешь же ещё этих мягких французских булок, да выпей чаю.' },
    { label: 'Russian (Цитрус)', text: 'В чащах юга жил-был цитрус? Да, но фальшивый экземпляр!' },
    { label: 'Numbers & Symbols', text: '0123456789 !@#$%^&*()_+-=[]{}|;:,.<>?' },
    { label: 'Alphabet Uppercase', text: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' },
    { label: 'Alphabet Lowercase', text: 'abcdefghijklmnopqrstuvwxyz' },
  ];

  let activePangramIndex = $state(0);
  let customText = $state('');
  let fontSize = $state(36);
  let lineHeight = $state(1.4);
  let letterSpacing = $state(0);
  let textAlign = $state<'left' | 'center' | 'right'>('left');

  let currentDisplayText = $derived(customText.trim() ? customText : PANGRAMS[activePangramIndex].text);

  const waterfallSizes = [12, 14, 18, 24, 36, 48, 60, 72];
  const quickSizes = [14, 18, 24, 36, 48, 72];
</script>

<div class="h-full flex flex-col overflow-y-auto p-6 space-y-8 select-text">
  <!-- Controls Toolbar -->
  <div class="p-4 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-4 select-none">
    <div class="flex flex-wrap items-center justify-between gap-4">
      <!-- Pangram quick picker -->
      <div class="flex items-center gap-2">
        <span class="text-xs font-semibold text-[var(--text-muted)]">Sample:</span>
        <Select
          value={activePangramIndex}
          options={PANGRAMS.map((p, idx) => ({ value: idx, label: p.label }))}
          ariaLabel="Choose specimen sample text"
          onChange={(val) => {
            activePangramIndex = Number(val);
            customText = '';
          }}
        />
      </div>

      <!-- Quick size buttons -->
      <div class="flex items-center gap-1">
        {#each quickSizes as s}
          <button
            type="button"
            onclick={() => (fontSize = s)}
            class="px-2 py-0.5 text-xs rounded transition-colors {fontSize === s ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'bg-[var(--bg-base)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
          >
            {s}px
          </button>
        {/each}
      </div>

      <!-- Alignment buttons -->
      <div class="flex items-center rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] p-0.5">
        <button
          type="button"
          onclick={() => (textAlign = 'left')}
          class="p-1 rounded transition-colors {textAlign === 'left' ? 'bg-[var(--bg-card-active)] text-[var(--accent)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          title="Align Left"
          aria-label="Align Left"
        >
          <AlignLeft size={14} />
        </button>
        <button
          type="button"
          onclick={() => (textAlign = 'center')}
          class="p-1 rounded transition-colors {textAlign === 'center' ? 'bg-[var(--bg-card-active)] text-[var(--accent)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          title="Align Center"
          aria-label="Align Center"
        >
          <AlignCenter size={14} />
        </button>
        <button
          type="button"
          onclick={() => (textAlign = 'right')}
          class="p-1 rounded transition-colors {textAlign === 'right' ? 'bg-[var(--bg-card-active)] text-[var(--accent)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          title="Align Right"
          aria-label="Align Right"
        >
          <AlignRight size={14} />
        </button>
      </div>
    </div>

    <!-- Sliders row -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-2 border-t border-[var(--border-subtle)]">
      <!-- Font Size Slider -->
      <div class="flex items-center gap-2 text-xs">
        <span class="text-[var(--text-muted)] w-10">Size:</span>
        <input
          type="range"
          min="12"
          max="120"
          bind:value={fontSize}
          class="flex-1 accent-[var(--accent)] cursor-pointer"
        />
        <span class="text-[var(--text-primary)] font-mono w-10 text-right">{fontSize}px</span>
      </div>

      <!-- Line Height Slider -->
      <div class="flex items-center gap-2 text-xs">
        <span class="text-[var(--text-muted)] w-14">Leading:</span>
        <input
          type="range"
          min="1.0"
          max="2.5"
          step="0.05"
          bind:value={lineHeight}
          class="flex-1 accent-[var(--accent)] cursor-pointer"
        />
        <span class="text-[var(--text-primary)] font-mono w-10 text-right">{lineHeight.toFixed(2)}</span>
      </div>

      <!-- Letter Spacing Slider -->
      <div class="flex items-center gap-2 text-xs">
        <span class="text-[var(--text-muted)] w-14">Tracking:</span>
        <input
          type="range"
          min="-2"
          max="10"
          step="0.5"
          bind:value={letterSpacing}
          class="flex-1 accent-[var(--accent)] cursor-pointer"
        />
        <span class="text-[var(--text-primary)] font-mono w-10 text-right">{letterSpacing}px</span>
      </div>
    </div>
  </div>

  <!-- Primary Specimen Display -->
  <section class="space-y-2">
    <div class="flex items-center justify-between text-xs text-[var(--text-muted)] select-none">
      <span class="font-semibold uppercase tracking-wider text-[11px]">Hero Specimen</span>
      <span class="font-mono">{fontSize}px / {lineHeight}</span>
    </div>

    <div class="p-6 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] overflow-hidden">
      <div
        contenteditable="true"
        class="outline-none text-[var(--text-primary)] transition-all cursor-text"
        style="
          font-family: {fontFamilyCss};
          font-size: {fontSize}px;
          line-height: {lineHeight};
          letter-spacing: {letterSpacing}px;
          text-align: {textAlign};
          font-weight: {font.weight};
          font-style: {font.is_italic ? 'italic' : 'normal'};
        "
      >
        {currentDisplayText}
      </div>
    </div>
  </section>

  <!-- Size waterfall section -->
  <section class="space-y-3">
    <div class="flex items-center justify-between text-xs text-[var(--text-muted)] select-none border-b border-[var(--border-subtle)] pb-2">
      <span class="font-semibold uppercase tracking-wider text-[11px]">Size Waterfall</span>
      <span class="text-[11px]">12pt to 72pt specimen ladder</span>
    </div>

    <div class="space-y-4 divide-y divide-[var(--border-subtle)]">
      {#each waterfallSizes as size}
        <div class="pt-3 flex items-baseline gap-4">
          <span class="w-12 text-right font-mono text-[11px] text-[var(--text-muted)] shrink-0 select-none">
            {size} pt
          </span>
          <div
            class="flex-1 overflow-hidden whitespace-nowrap text-ellipsis text-[var(--text-primary)]"
            style="
              font-family: {fontFamilyCss};
              font-size: {size}px;
              font-weight: {font.weight};
              font-style: {font.is_italic ? 'italic' : 'normal'};
            "
          >
            {currentDisplayText}
          </div>
        </div>
      {/each}
    </div>
  </section>

  <!-- Complete Alphabet & Numbers Sets -->
  <section class="space-y-4 pt-4 border-t border-[var(--border-subtle)]">
    <span class="font-semibold uppercase tracking-wider text-[11px] text-[var(--text-muted)] select-none block">
      Alphabet & Characters
    </span>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Latin Set -->
      <div class="p-4 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-2">
        <span class="text-[11px] font-semibold text-[var(--text-muted)] uppercase block select-none">Latin Alphabet</span>
        <div class="text-xl text-[var(--text-primary)] tracking-wide" style="font-family: {fontFamilyCss};">
          A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
        </div>
        <div class="text-lg text-[var(--text-secondary)] tracking-wide" style="font-family: {fontFamilyCss};">
          a b c d e f g h i j k l m n o p q r s t u v w x y z
        </div>
      </div>

      <!-- Numerals & Punctuation -->
      <div class="p-4 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-2">
        <span class="text-[11px] font-semibold text-[var(--text-muted)] uppercase block select-none">Numerals & Punctuation</span>
        <div class="text-xl text-[var(--text-primary)] tracking-wider" style="font-family: {fontFamilyCss};">
          0 1 2 3 4 5 6 7 8 9
        </div>
        <div class="text-lg text-[var(--text-secondary)] tracking-wider" style="font-family: {fontFamilyCss};">
          ! ? @ # $ % & * ( ) [ ] &#123; &#125; , . : ; " ' - + = /
        </div>
      </div>

      <!-- Cyrillic Set -->
      <div class="p-4 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-2 md:col-span-2">
        <span class="text-[11px] font-semibold text-[var(--text-muted)] uppercase block select-none">Cyrillic Alphabet</span>
        <div class="text-xl text-[var(--text-primary)] tracking-wide" style="font-family: {fontFamilyCss};">
          А Б В Г Д Е Ё Ж З И Й К Л М Н О П Р С Т У Ф Х Ц Ч Ш Щ Ъ Ы Ь Э Ю Я
        </div>
        <div class="text-lg text-[var(--text-secondary)] tracking-wide" style="font-family: {fontFamilyCss};">
          а б в г д е ё ж з и й к л м н о п р с т у ф х ц ч ш щ ъ ы ь э ю я
        </div>
      </div>
    </div>
  </section>
</div>
