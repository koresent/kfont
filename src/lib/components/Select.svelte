<script lang="ts">
  import { ChevronDown, Check } from '@lucide/svelte';

  let {
    value,
    options = [],
    ariaLabel = 'Select option',
    align = 'left',
    onChange,
  }: {
    value: any;
    options: { value: any; label: string }[];
    ariaLabel?: string;
    align?: 'left' | 'right';
    onChange: (val: any) => void;
  } = $props();

  let isOpen = $state(false);
  let selectButtonEl = $state<HTMLButtonElement | null>(null);

  let selectedOption = $derived(
    options.find((o) => o.value === value) || options[0] || { value: '', label: '' }
  );

  function handleSelect(val: any) {
    onChange(val);
    isOpen = false;
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      isOpen = false;
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      const idx = options.findIndex((o) => o.value === value);
      if (idx < options.length - 1) {
        onChange(options[idx + 1].value);
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      const idx = options.findIndex((o) => o.value === value);
      if (idx > 0) {
        onChange(options[idx - 1].value);
      }
    }
  }
</script>

<svelte:window
  onclick={(e) => {
    if (isOpen && selectButtonEl && !selectButtonEl.contains(e.target as Node)) {
      isOpen = false;
    }
  }}
/>

<div class="relative inline-block text-left">
  <!-- Dropdown Trigger Button -->
  <button
    bind:this={selectButtonEl}
    type="button"
    onkeydown={handleKeyDown}
    onclick={() => (isOpen = !isOpen)}
    aria-haspopup="listbox"
    aria-expanded={isOpen}
    aria-label={ariaLabel}
    class="flex items-center justify-between gap-2 px-2.5 py-1 text-xs rounded-md bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] hover:border-[var(--border-strong)] hover:bg-[var(--bg-card-hover)] transition-all outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] cursor-pointer select-none"
  >
    <span class="truncate max-w-[160px]">{selectedOption.label}</span>
    <ChevronDown size={14} class="text-[var(--text-muted)] transition-transform duration-150 {isOpen ? 'rotate-180 text-[var(--accent)]' : ''}" />
  </button>

  <!-- Themed Popover Menu -->
  {#if isOpen}
    <div
      role="listbox"
      aria-label={ariaLabel}
      class="absolute {align === 'right' ? 'right-0' : 'left-0'} top-full mt-1 z-50 min-w-full w-max max-h-60 overflow-y-auto rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] shadow-xl p-1 space-y-0.5 animate-in fade-in select-none"
    >
      {#each options as opt}
        {@const isSelected = opt.value === value}
        <button
          type="button"
          role="option"
          aria-selected={isSelected}
          onclick={() => handleSelect(opt.value)}
          class="w-full flex items-center justify-between gap-3 px-2.5 py-1.5 text-xs rounded-md text-left transition-colors cursor-pointer outline-none {isSelected ? 'bg-[var(--bg-card-active)] text-[var(--accent)] font-semibold' : 'text-[var(--text-primary)] hover:bg-[var(--bg-card-hover)]'}"
        >
          <span class="truncate">{opt.label}</span>
          {#if isSelected}
            <Check size={13} class="text-[var(--accent)] shrink-0" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
