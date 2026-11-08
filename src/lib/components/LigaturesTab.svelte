<script lang="ts">
  import {
    Code,
    Columns2,
    Copy,
    Check,
    ToggleLeft,
    ToggleRight,
    Search,
    RotateCcw,
    Sparkles,
  } from '@lucide/svelte';
  import type { FontSummary } from '$lib/types';

  let {
    font,
    fontFamilyCss = 'sans-serif',
  }: {
    font: FontSummary;
    fontFamilyCss: string;
  } = $props();

  // View state
  let viewMode = $state<'catalog' | 'playground' | 'diff'>('catalog');
  let ligaturesEnabled = $state(true);
  let fontSize = $state(18);
  let cardFontSize = $state(28);
  let searchQuery = $state('');
  let selectedCategory = $state('all');
  let selectedSnippet = $state('typescript');
  let copiedText = $state<string | null>(null);

  // Common programming ligatures categorized with guaranteed unique IDs
  const ligatureCatalog = [
    // Arrows & Pointers
    { id: 'arr-thin-right', symbol: '->', name: 'Thin right arrow', category: 'arrows' },
    { id: 'arr-thin-left', symbol: '<-', name: 'Thin left arrow', category: 'arrows' },
    { id: 'arr-fat-right', symbol: '=>', name: 'Fat right arrow', category: 'arrows' },
    { id: 'arr-double-bidir', symbol: '<=>', name: 'Double bidirectional', category: 'arrows' },
    { id: 'arr-long-right', symbol: '-->', name: 'Long right arrow', category: 'arrows' },
    { id: 'arr-long-left', symbol: '<--', name: 'Long left arrow', category: 'arrows' },
    { id: 'arr-long-fat-right', symbol: '==>', name: 'Long fat right arrow', category: 'arrows' },
    { id: 'arr-long-fat-left', symbol: '<==', name: 'Long fat left arrow', category: 'arrows' },
    { id: 'arr-double-head-right', symbol: '->>', name: 'Double head arrow', category: 'arrows' },
    { id: 'arr-double-head-left', symbol: '<<-', name: 'Double head left arrow', category: 'arrows' },
    { id: 'arr-wiggle-right', symbol: '~>', name: 'Wiggle arrow', category: 'arrows' },
    { id: 'arr-wiggle-left', symbol: '<~', name: 'Left wiggle arrow', category: 'arrows' },
    { id: 'arr-pipe-right', symbol: '|>', name: 'Pipeline', category: 'arrows' },
    { id: 'arr-pipe-left', symbol: '<|', name: 'Left pipeline', category: 'arrows' },

    // Comparisons
    { id: 'cmp-eq', symbol: '==', name: 'Equality', category: 'comparisons' },
    { id: 'cmp-strict-eq', symbol: '===', name: 'Strict equality', category: 'comparisons' },
    { id: 'cmp-neq', symbol: '!=', name: 'Inequality', category: 'comparisons' },
    { id: 'cmp-strict-neq', symbol: '!==', name: 'Strict inequality', category: 'comparisons' },
    { id: 'cmp-lte', symbol: '<=', name: 'Less or equal', category: 'comparisons' },
    { id: 'cmp-gte', symbol: '>=', name: 'Greater or equal', category: 'comparisons' },
    { id: 'cmp-diamond', symbol: '<>', name: 'Diamond / Not equal', category: 'comparisons' },
    { id: 'cmp-overlap', symbol: '><', name: 'Overlap', category: 'comparisons' },
    { id: 'cmp-regex-match', symbol: '=~', name: 'Regex match', category: 'comparisons' },
    { id: 'cmp-regex-nonmatch', symbol: '!~', name: 'Regex non-match', category: 'comparisons' },
    { id: 'cmp-approx-eq', symbol: '~=', name: 'Approx equal / Lua neq', category: 'comparisons' },
    { id: 'cmp-haskell-neq', symbol: '/=', name: 'Haskell not equal', category: 'comparisons' },

    // Assignment & Logic
    { id: 'logic-walrus', symbol: ':=', name: 'Walrus / Definition', category: 'logic' },
    { id: 'logic-bnf', symbol: '::=', name: 'BNF assignment', category: 'logic' },
    { id: 'logic-add-assign', symbol: '+=', name: 'Add assign', category: 'logic' },
    { id: 'logic-sub-assign', symbol: '-=', name: 'Sub assign', category: 'logic' },
    { id: 'logic-mul-assign', symbol: '*=', name: 'Mul assign', category: 'logic' },
    { id: 'logic-div-assign', symbol: '/=', name: 'Div assign', category: 'logic' },
    { id: 'logic-mod-assign', symbol: '%=', name: 'Mod assign', category: 'logic' },
    { id: 'logic-and-assign', symbol: '&=', name: 'Bitwise AND assign', category: 'logic' },
    { id: 'logic-or-assign', symbol: '|=', name: 'Bitwise OR assign', category: 'logic' },
    { id: 'logic-xor-assign', symbol: '^=', name: 'Bitwise XOR assign', category: 'logic' },
    { id: 'logic-and', symbol: '&&', name: 'Logical AND', category: 'logic' },
    { id: 'logic-or', symbol: '||', name: 'Logical OR', category: 'logic' },
    { id: 'logic-nullish', symbol: '??', name: 'Nullish coalescing', category: 'logic' },
    { id: 'logic-opt-chain', symbol: '?.', name: 'Optional chaining', category: 'logic' },
    { id: 'logic-not-not', symbol: '!!', name: 'Double negation', category: 'logic' },

    // Delimiters & Comments
    { id: 'comm-block-open', symbol: '/*', name: 'Block comment start', category: 'comments' },
    { id: 'comm-block-close', symbol: '*/', name: 'Block comment end', category: 'comments' },
    { id: 'comm-html-open', symbol: '<!--', name: 'HTML comment open', category: 'comments' },
    { id: 'comm-html-close', symbol: '-->', name: 'HTML comment close', category: 'comments' },
    { id: 'comm-doc', symbol: '///', name: 'Doc comment', category: 'comments' },
    { id: 'comm-line', symbol: '//', name: 'Line comment', category: 'comments' },
    { id: 'comm-haskell-open', symbol: '{-', name: 'Haskell comment open', category: 'comments' },
    { id: 'comm-haskell-close', symbol: '-}', name: 'Haskell comment close', category: 'comments' },

    // Functional & Ranges
    { id: 'func-colon-double', symbol: '::', name: 'Path separator / Type', category: 'functional' },
    { id: 'func-colon-triple', symbol: ':::', name: 'Triple colon', category: 'functional' },
    { id: 'func-range-excl', symbol: '..', name: 'Exclusive range', category: 'functional' },
    { id: 'func-ellipsis', symbol: '...', name: 'Spread / Ellipsis', category: 'functional' },
    { id: 'func-range-incl', symbol: '..=', name: 'Inclusive range', category: 'functional' },
    { id: 'func-bind', symbol: '>>=', name: 'Monadic bind', category: 'functional' },
    { id: 'func-bind-rev', symbol: '=<<', name: 'Reversed bind', category: 'functional' },
    { id: 'func-fmap', symbol: '<$>', name: 'Functor fmap', category: 'functional' },
    { id: 'func-apply', symbol: '<*>', name: 'Applicative apply', category: 'functional' },
    { id: 'func-alt', symbol: '<|>', name: 'Alternative or', category: 'functional' },
    { id: 'func-shl', symbol: '<<', name: 'Left shift', category: 'functional' },
    { id: 'func-shr', symbol: '>>', name: 'Right shift', category: 'functional' },

    // Boxes & Symbols
    { id: 'sym-box-empty', symbol: '[ ]', name: 'Unchecked task', category: 'brackets' },
    { id: 'sym-box-checked', symbol: '[x]', name: 'Checked task', category: 'brackets' },
    { id: 'sym-pragma-open', symbol: '{-#', name: 'Pragma open', category: 'brackets' },
    { id: 'sym-pragma-close', symbol: '#-}', name: 'Pragma close', category: 'brackets' },
    { id: 'sym-hash-triple', symbol: '###', name: 'Triple hash', category: 'brackets' },
    { id: 'sym-hash-double', symbol: '##', name: 'Double hash', category: 'brackets' },
  ];

  const categories = [
    { value: 'all', label: 'All Ligatures' },
    { value: 'arrows', label: 'Arrows & Pointers' },
    { value: 'comparisons', label: 'Comparisons' },
    { value: 'logic', label: 'Assignment & Logic' },
    { value: 'comments', label: 'Comments & HTML' },
    { value: 'functional', label: 'Functional & Ranges' },
    { value: 'brackets', label: 'Boxes & Symbols' },
  ];

  let filteredCatalog = $derived.by(() => {
    let list = ligatureCatalog;
    if (selectedCategory !== 'all') {
      list = list.filter((item) => item.category === selectedCategory);
    }
    if (searchQuery.trim()) {
      const q = searchQuery.trim().toLowerCase();
      list = list.filter(
        (item) => item.symbol.toLowerCase().includes(q) || item.name.toLowerCase().includes(q)
      );
    }
    return list;
  });

  const codeSnippets: Record<string, string> = {
    typescript: `// TypeScript & Modern Web Ligatures Test
const verifySession = async <T>(token: string | null): Promise<boolean> => {
  if (token === null || token === undefined) return false;
  const isValid = token?.length >= 32 && token !== "expired";
  
  // Logical and nullish assignments
  let flags = 0;
  flags |= 0b0010;
  const timeout ??= 5000;
  
  // Function composition & arrow functions
  const pipeline = (val: number) => val * 2;
  return isValid && (flags !== 0) && timeout <= 10000;
};

// HTML & JSX comments: <!-- ligature check -->
/* Block comment test: !==, ===, <=, >=, =>, -> */`,

    rust: `// Rust Idiomatic Ligatures Test
pub fn process_data<T: Ord + Clone>(items: &[T]) -> Result<Option<T>, &'static str> {
    let mut count = 0;
    for i in 0..=items.len() {
        if i >= 100 && i != 256 {
            count += 1;
        }
    }
    
    // Pattern matching & closures
    let mapper = |x: i32| -> i32 {
        if x <= 0 || x == 42 { 0 } else { x * 2 }
    };
    
    match items.first() {
        Some(val) => Ok(Some(val.clone())),
        None => Err("Empty slice"),
    }
}`,

    python: `# Python 3.12+ Ligatures Test
def analyze_points(coords: list[tuple[int, int]]) -> dict[str, float]:
    """Calculate geometric metrics: -> float."""
    if not coords or len(coords) == 0:
        return {"distance": 0.0}
        
    total_dist = 0.0
    for (x, y) in coords:
        if x != 0 and y >= 0:
            total_dist += (x ** 2 + y ** 2) ** 0.5
            
    # Walrus operator := and arithmetic
    while (n := len(coords)) > 10:
        coords.pop()
        
    return {"mean": total_dist // len(coords) if total_dist != 0 else 0.0}`,

    haskell: `-- Haskell Functional Ligatures Test
module LigatureCheck where

processStream :: [Int] -> Maybe Int
processStream xs = xs >>= \\x ->
  if x >= 0 && x /= 999
    then Just (x * 2)
    else Nothing

-- Applicative and functor operators: <$>, <*>, <|>, ::, ..
pipeline :: Maybe (Int -> Int) -> Maybe Int -> Maybe Int
pipeline f val = f <*> val <|> Just 0`,

    markdown: `<!-- Markdown & Ascii Diagrams Test -->
# Ligature Overview: ===, !==, <=, >=

### Task Checkboxes
- [ ] Incomplete task item
- [x] Completed task item

### Flowchart Arrows
Start ==> Process --> Check <==> Validate --> Done
Sub-routine: <<-- Revert || Re-try -->>

/* Multi-line comment block: */// End of file`,
  };

  let playgroundText = $state(codeSnippets.typescript);

  function handleSelectSnippet(snipKey: string) {
    selectedSnippet = snipKey;
    playgroundText = codeSnippets[snipKey] || '';
  }

  function copySymbol(sym: string) {
    if (navigator?.clipboard?.writeText) {
      navigator.clipboard.writeText(sym).catch(() => {});
    }
    copiedText = sym;
    setTimeout(() => {
      copiedText = null;
    }, 1500);
  }
</script>

<div class="h-full flex flex-col bg-[var(--bg-base)] overflow-hidden">
  <!-- Controls Header Toolbar -->
  <div class="p-3 border-b border-[var(--border-subtle)] bg-[var(--bg-card)] flex flex-wrap items-center justify-between gap-3 shrink-0 select-none">
    <!-- View Mode Switcher -->
    <div class="flex items-center gap-1 bg-[var(--bg-base)] p-1 rounded-lg border border-[var(--border-subtle)]">
      <button
        type="button"
        onclick={() => (viewMode = 'catalog')}
        class="px-2.5 py-1 text-xs rounded-md font-medium flex items-center gap-1.5 transition-colors cursor-pointer {viewMode === 'catalog' ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}"
      >
        <Sparkles size={13} />
        <span>Ligature Grid</span>
      </button>

      <button
        type="button"
        onclick={() => (viewMode = 'playground')}
        class="px-2.5 py-1 text-xs rounded-md font-medium flex items-center gap-1.5 transition-colors cursor-pointer {viewMode === 'playground' ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}"
      >
        <Code size={13} />
        <span>Code Editor</span>
      </button>

      <button
        type="button"
        onclick={() => (viewMode = 'diff')}
        class="px-2.5 py-1 text-xs rounded-md font-medium flex items-center gap-1.5 transition-colors cursor-pointer {viewMode === 'diff' ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'}"
      >
        <Columns2 size={13} />
        <span>Side-by-Side Diff</span>
      </button>
    </div>

    <!-- Middle & Right Controls -->
    <div class="flex flex-wrap items-center gap-3 text-xs">
      <!-- Ligatures Toggle Pill -->
      <button
        type="button"
        onclick={() => (ligaturesEnabled = !ligaturesEnabled)}
        class="flex items-center gap-2 px-3 py-1 rounded-md border transition-colors cursor-pointer {ligaturesEnabled ? 'border-[var(--accent)] bg-[var(--bg-card-active)] text-[var(--accent)] font-semibold' : 'border-[var(--border-subtle)] bg-[var(--bg-base)] text-[var(--text-muted)]'}"
        title="Toggle font-variant-ligatures: normal / none"
      >
        {#if ligaturesEnabled}
          <ToggleRight size={16} class="text-[var(--accent)]" />
          <span>Ligatures: ON</span>
        {:else}
          <ToggleLeft size={16} class="text-[var(--text-muted)]" />
          <span>Ligatures: OFF</span>
        {/if}
      </button>

      <!-- Font Size Slider -->
      <div class="flex items-center gap-1.5" title="Font size">
        <span class="text-[11px] text-[var(--text-muted)]">Size:</span>
        <input
          type="range"
          min="12"
          max={viewMode === 'catalog' ? 48 : 32}
          step="1"
          value={viewMode === 'catalog' ? cardFontSize : fontSize}
          oninput={(e) => {
            const val = Number((e.target as HTMLInputElement).value);
            if (viewMode === 'catalog') {
              cardFontSize = val;
            } else {
              fontSize = val;
            }
          }}
          class="w-20 accent-[var(--accent)] cursor-pointer"
        />
        <span class="text-[10px] text-[var(--text-muted)] tabular-nums w-4">
          {viewMode === 'catalog' ? cardFontSize : fontSize}
        </span>
      </div>
    </div>
  </div>

  <!-- Content Areas -->
  {#if viewMode === 'catalog'}
    <!-- Catalog Sub-Toolbar: Category & Search -->
    <div class="p-3 border-b border-[var(--border-subtle)] bg-[var(--bg-base)] flex flex-wrap items-center justify-between gap-3 shrink-0 select-none">
      <!-- Category Pills -->
      <div class="flex flex-wrap items-center gap-1">
        {#each categories as cat (cat.value)}
          <button
            type="button"
            onclick={() => (selectedCategory = cat.value)}
            class="px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors cursor-pointer {selectedCategory === cat.value ? 'bg-[var(--accent)] text-[var(--accent-text)]' : 'bg-[var(--bg-card)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)] hover:text-[var(--text-primary)] border border-[var(--border-subtle)]'}"
          >
            {cat.label}
          </button>
        {/each}
      </div>

      <!-- Quick Search -->
      <div class="relative flex items-center">
        <Search size={14} class="absolute left-2.5 text-[var(--text-muted)] pointer-events-none" />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Filter ligatures (e.g. ===, ->, !=)..."
          class="pl-8 pr-3 py-1 text-xs rounded-md bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none w-56"
        />
      </div>
    </div>

    <!-- Ligatures Cards Grid -->
    <div class="flex-1 overflow-y-auto p-4 select-none">
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
        {#each filteredCatalog as item (item.id)}
          {@const isCopied = copiedText === item.symbol}
          <div
            role="button"
            tabindex="0"
            onclick={() => copySymbol(item.symbol)}
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                copySymbol(item.symbol);
              }
            }}
            class="p-3.5 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-card)] hover:border-[var(--accent)] hover:shadow-md transition-all flex flex-col justify-between gap-3 text-center group cursor-pointer relative"
          >
            <!-- Copy icon overlay on hover -->
            <div class="absolute top-2 right-2 text-[var(--text-muted)] group-hover:text-[var(--accent)] transition-colors">
              {#if isCopied}
                <Check size={13} class="text-[var(--accent)]" />
              {:else}
                <Copy size={13} class="opacity-0 group-hover:opacity-100 transition-opacity" />
              {/if}
            </div>

            <!-- Big Ligature Preview -->
            <div
              class="h-16 flex items-center justify-center tracking-normal text-[var(--text-primary)] overflow-hidden"
              style="font-family: {fontFamilyCss}, monospace; font-size: {cardFontSize}px; font-weight: {font.weight}; font-style: {font.is_italic ? 'italic' : 'normal'}; font-variant-ligatures: {ligaturesEnabled ? 'normal' : 'none'}; font-feature-settings: 'calt' {ligaturesEnabled ? 1 : 0}, 'liga' {ligaturesEnabled ? 1 : 0}, 'dlig' {ligaturesEnabled ? 1 : 0};"
            >
              {item.symbol}
            </div>

            <!-- Sub info: Symbol characters and comparison -->
            <div class="pt-2 border-t border-[var(--border-subtle)] flex flex-col items-center gap-1">
              <span class="font-mono text-xs font-semibold text-[var(--text-primary)] bg-[var(--bg-base)] px-2 py-0.5 rounded border border-[var(--border-subtle)]">
                {item.symbol}
              </span>
              <span class="text-[10px] text-[var(--text-muted)] truncate max-w-full" title={item.name}>
                {item.name}
              </span>
            </div>
          </div>
        {/each}
      </div>

      {#if filteredCatalog.length === 0}
        <div class="p-12 text-center text-xs text-[var(--text-muted)] flex flex-col items-center justify-center gap-2">
          <p class="font-medium text-[var(--text-primary)]">No matching ligatures found</p>
          <p class="text-[11px]">Try clearing your search query or selecting All Ligatures</p>
        </div>
      {/if}
    </div>

  {:else if viewMode === 'playground'}
    <!-- Code Editor Mode -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Language Snippet Selector Bar -->
      <div class="p-2.5 border-b border-[var(--border-subtle)] bg-[var(--bg-base)] flex items-center justify-between gap-3 shrink-0 select-none text-xs">
        <div class="flex items-center gap-2">
          <span class="text-[var(--text-muted)] font-medium">Sample Code:</span>
          <div class="flex items-center gap-1">
            {#each ['typescript', 'rust', 'python', 'haskell', 'markdown'] as snip (snip)}
              <button
                type="button"
                onclick={() => handleSelectSnippet(snip)}
                class="px-2.5 py-1 rounded text-xs capitalize transition-colors cursor-pointer {selectedSnippet === snip ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'bg-[var(--bg-card)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
              >
                {snip}
              </button>
            {/each}
          </div>
        </div>

        <button
          type="button"
          onclick={() => handleSelectSnippet(selectedSnippet)}
          class="flex items-center gap-1.5 px-2 py-1 rounded text-xs text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors cursor-pointer"
          title="Reset to default sample code"
        >
          <RotateCcw size={12} />
          <span>Reset code</span>
        </button>
      </div>

      <!-- Editable Code Area -->
      <div class="flex-1 p-4 bg-[var(--bg-base)] overflow-hidden flex flex-col">
        <textarea
          bind:value={playgroundText}
          spellcheck="false"
          class="w-full flex-1 p-4 rounded-lg bg-[var(--bg-card)] text-[var(--text-primary)] border border-[var(--border-subtle)] focus:border-[var(--accent)] outline-none resize-none leading-relaxed transition-all shadow-inner"
          style="font-family: {fontFamilyCss}, monospace; font-size: {fontSize}px; font-weight: {font.weight}; font-style: {font.is_italic ? 'italic' : 'normal'}; font-variant-ligatures: {ligaturesEnabled ? 'normal' : 'none'}; font-feature-settings: 'calt' {ligaturesEnabled ? 1 : 0}, 'liga' {ligaturesEnabled ? 1 : 0}, 'dlig' {ligaturesEnabled ? 1 : 0}; tab-size: 2;"
          aria-label="Code Playground"
        ></textarea>
      </div>
    </div>

  {:else if viewMode === 'diff'}
    <!-- Side-by-Side Diff: Left is Ligatures ON, Right is Ligatures OFF -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Snippet Bar -->
      <div class="p-2.5 border-b border-[var(--border-subtle)] bg-[var(--bg-base)] flex items-center justify-between gap-3 shrink-0 select-none text-xs">
        <div class="flex items-center gap-2">
          <span class="text-[var(--text-muted)] font-medium">Snippet:</span>
          <div class="flex items-center gap-1">
            {#each ['typescript', 'rust', 'python', 'haskell'] as snip (snip)}
              <button
                type="button"
                onclick={() => handleSelectSnippet(snip)}
                class="px-2.5 py-1 rounded text-xs capitalize transition-colors cursor-pointer {selectedSnippet === snip ? 'bg-[var(--accent)] text-[var(--accent-text)] font-semibold' : 'bg-[var(--bg-card)] text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
              >
                {snip}
              </button>
            {/each}
          </div>
        </div>

        <span class="text-[11px] text-[var(--text-muted)]">Type on either side to test simultaneously</span>
      </div>

      <div class="flex-1 grid grid-cols-2 divide-x divide-[var(--border-subtle)] overflow-hidden">
        <!-- Left: Ligatures ENABLED -->
        <div class="flex flex-col h-full bg-[var(--bg-card)] p-4 overflow-hidden">
          <div class="flex items-center justify-between pb-2 text-xs font-semibold text-[var(--accent)] select-none border-b border-[var(--border-subtle)] mb-2">
            <span>Ligatures: Enabled</span>
            <span class="text-[10px] font-mono text-[var(--text-muted)] font-normal">calt 1, liga 1</span>
          </div>
          <textarea
            bind:value={playgroundText}
            spellcheck="false"
            class="w-full flex-1 bg-transparent text-[var(--text-primary)] outline-none resize-none leading-relaxed"
            style="font-family: {fontFamilyCss}, monospace; font-size: {fontSize}px; font-weight: {font.weight}; font-style: {font.is_italic ? 'italic' : 'normal'}; font-variant-ligatures: normal; font-feature-settings: 'calt' 1, 'liga' 1, 'dlig' 1; tab-size: 2;"
            aria-label="Ligatures Enabled View"
          ></textarea>
        </div>

        <!-- Right: Ligatures DISABLED -->
        <div class="flex flex-col h-full bg-[var(--bg-base)] p-4 overflow-hidden">
          <div class="flex items-center justify-between pb-2 text-xs font-semibold text-[var(--text-secondary)] select-none border-b border-[var(--border-subtle)] mb-2">
            <span>Ligatures: Disabled</span>
            <span class="text-[10px] font-mono text-[var(--text-muted)] font-normal">calt 0, liga 0</span>
          </div>
          <textarea
            bind:value={playgroundText}
            spellcheck="false"
            class="w-full flex-1 bg-transparent text-[var(--text-primary)] outline-none resize-none leading-relaxed"
            style="font-family: {fontFamilyCss}, monospace; font-size: {fontSize}px; font-weight: {font.weight}; font-style: {font.is_italic ? 'italic' : 'normal'}; font-variant-ligatures: none; font-feature-settings: 'calt' 0, 'liga' 0, 'dlig' 0; tab-size: 2;"
            aria-label="Ligatures Disabled View"
          ></textarea>
        </div>
      </div>
    </div>
  {/if}
</div>
