<script lang="ts">
  import type { FontSummary } from '$lib/types';

  let {
    font,
    fontFamilyCss,
  }: {
    font: FontSummary;
    fontFamilyCss: string;
  } = $props();

  interface OpenTypeFeature {
    tag: string;
    name: string;
    description: string;
    sampleText: string;
    enabled: boolean;
  }

  let features = $state<OpenTypeFeature[]>([
    {
      tag: 'liga',
      name: 'Standard Ligatures',
      description: 'Replaces character sequences like fi, fl, ffi, ffl with single unified glyphs.',
      sampleText: 'fi fl ffi ffl fj ff - difficult office flight',
      enabled: true,
    },
    {
      tag: 'dlig',
      name: 'Discretionary Ligatures',
      description: 'Decorative ligatures such as st, ct, sp, Th.',
      sampleText: 'Strict action, special star, aesthetic',
      enabled: false,
    },
    {
      tag: 'kern',
      name: 'Kerning',
      description: 'Adjusts spacing between specific glyph pairs (e.g. AV, WA, To).',
      sampleText: 'AV WA To Te VA Wo WAITING WATER',
      enabled: true,
    },
    {
      tag: 'smcp',
      name: 'Small Capitals',
      description: 'Renders lowercase letters as uppercase glyphs proportioned to the x-height.',
      sampleText: 'Typography and Graphic Design 2026',
      enabled: false,
    },
    {
      tag: 'onum',
      name: 'Oldstyle Figures',
      description: 'Numbers designed with varying heights and descenders (lining vs oldstyle).',
      sampleText: '0123456789 - In 1789, 452 items cost $1,999.00',
      enabled: false,
    },
    {
      tag: 'tnum',
      name: 'Tabular Figures',
      description: 'Sets numerals with uniform widths for financial and data columns.',
      sampleText: '$1,234.50\n$9,876.10\n$0,042.99',
      enabled: false,
    },
    {
      tag: 'frac',
      name: 'Fractions',
      description: 'Converts slashed numerals (1/2, 3/4) into true diagonal typographic fractions.',
      sampleText: '1/2 1/4 3/4 1/3 2/3 5/8 7/8 9/10',
      enabled: true,
    },
    {
      tag: 'zero',
      name: 'Slashed Zero',
      description: 'Renders the numeral 0 with a diagonal slash to distinguish from capital O.',
      sampleText: '0 O 000 OOO 10203040 $500',
      enabled: false,
    },
    {
      tag: 'calt',
      name: 'Contextual Alternates',
      description: 'Adapts glyph shapes depending on neighboring characters (e.g. script joins or code arrows).',
      sampleText: '-> => != == === >= <= <!-- -->',
      enabled: true,
    },
  ]);

  let activeFeatureSettings = $derived(
    features.map((f) => `"${f.tag}" ${f.enabled ? 1 : 0}`).join(', ')
  );

  let customSandboxText = $state('fi fl ffi ffl 1/2 3/4 $1,234.50 0 O -> !=');
</script>

<div class="h-full flex flex-col overflow-y-auto p-6 space-y-6 select-text">
  <!-- Interactive Sandbox -->
  <section class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-3">
    <div class="flex items-center justify-between text-xs select-none">
      <span class="font-semibold uppercase tracking-wider text-[var(--accent)]">
        Live OpenType Playground
      </span>
      <span class="font-mono text-[var(--text-muted)] text-[11px] truncate max-w-md">
        font-feature-settings: {activeFeatureSettings}
      </span>
    </div>

    <input
      type="text"
      bind:value={customSandboxText}
      class="w-full px-3 py-2 text-xs rounded bg-[var(--bg-base)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none"
      placeholder="Type custom text to test active OpenType features..."
    />

    <div
      class="p-4 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-2xl text-[var(--text-primary)] transition-all leading-normal"
      style="
        font-family: {fontFamilyCss};
        font-weight: {font.weight};
        font-style: {font.is_italic ? 'italic' : 'normal'};
        font-feature-settings: {activeFeatureSettings};
      "
    >
      {customSandboxText}
    </div>
  </section>

  <!-- Features Toggle Cards List -->
  <section class="space-y-3 select-none">
    <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)] block">
      OpenType Feature Toggles
    </span>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each features as feature}
        <div class="p-4 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-3">
          <div class="flex items-start justify-between gap-3">
            <div>
              <div class="flex items-center gap-2">
                <h3 class="text-xs font-bold text-[var(--text-primary)]">{feature.name}</h3>
                <span class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-[var(--bg-base)] text-[var(--accent)] font-semibold">
                  {feature.tag}
                </span>
              </div>
              <p class="text-[11px] text-[var(--text-muted)] mt-1 leading-snug">{feature.description}</p>
            </div>

            <!-- Toggle Switch -->
            <button
              type="button"
              onclick={() => (feature.enabled = !feature.enabled)}
              class="w-9 h-5 rounded-full transition-colors relative shrink-0 focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none {feature.enabled ? 'bg-[var(--accent)]' : 'bg-[var(--border-strong)]'}"
              role="switch"
              aria-checked={feature.enabled}
              aria-label={`Toggle ${feature.name}`}
            >
              <span
                class="absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white transition-transform {feature.enabled ? 'translate-x-4' : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <!-- Feature comparison visual: OFF vs ON -->
          <div class="pt-2 border-t border-[var(--border-subtle)] grid grid-cols-2 gap-2 text-xs">
            <div class="p-2 rounded bg-[var(--bg-base)]">
              <span class="text-[10px] text-[var(--text-muted)] uppercase block mb-1">Off:</span>
              <div
                class="text-sm truncate text-[var(--text-secondary)]"
                style="
                  font-family: {fontFamilyCss};
                  font-weight: {font.weight};
                  font-style: {font.is_italic ? 'italic' : 'normal'};
                  font-feature-settings: '{feature.tag}' 0;
                "
              >
                {feature.sampleText}
              </div>
            </div>

            <div class="p-2 rounded bg-[var(--bg-base)] border {feature.enabled ? 'border-[var(--accent)]/40' : 'border-transparent'}">
              <span class="text-[10px] text-[var(--accent)] uppercase font-semibold block mb-1">On:</span>
              <div
                class="text-sm truncate text-[var(--text-primary)] font-medium"
                style="
                  font-family: {fontFamilyCss};
                  font-weight: {font.weight};
                  font-style: {font.is_italic ? 'italic' : 'normal'};
                  font-feature-settings: '{feature.tag}' 1;
                "
              >
                {feature.sampleText}
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  </section>
</div>
