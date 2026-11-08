<script lang="ts">
  import { Folder, FolderPlus, FilePlus, Settings, RefreshCw, Type, Hash, Sparkles, Trash2 } from '@lucide/svelte';

  let {
    folders = [],
    selectedFolderId = 'system',
    filterCategory = 'all',
    totalFontsCount = 0,
    filteredCount = 0,
    isScanning = false,
    onSelectFolder,
    onAddFolder,
    onAddFiles,
    onRemoveFolder,
    onSelectCategory,
    onRefresh,
    onOpenSettings,
  }: {
    folders: { id: string; name: string; path: string; count: number }[];
    selectedFolderId: string;
    filterCategory: string;
    totalFontsCount: number;
    filteredCount: number;
    isScanning: boolean;
    onSelectFolder: (id: string) => void;
    onAddFolder: () => void;
    onAddFiles: () => void;
    onRemoveFolder: (id: string) => void;
    onSelectCategory: (cat: string) => void;
    onRefresh: () => void;
    onOpenSettings: () => void;
  } = $props();

  const categories = [
    { id: 'all', label: 'All Fonts' },
    { id: 'sans', label: 'Sans-Serif' },
    { id: 'serif', label: 'Serif' },
    { id: 'mono', label: 'Monospaced' },
    { id: 'italic', label: 'Italics' },
  ];
</script>

<aside class="w-64 flex flex-col h-full bg-[var(--bg-sidebar)] border-r border-[var(--border-subtle)] select-none shrink-0" aria-label="Sidebar Navigation">
  <!-- Brand Header -->
  <div class="h-14 px-4 flex items-center justify-between border-b border-[var(--border-subtle)]">
    <div class="flex items-center gap-2.5">
      <img
        src="/logo.png"
        alt="kfont logo"
        class="w-7 h-7 rounded-md object-contain shrink-0 shadow-xs"
        draggable="false"
      />
      <div>
        <h1 class="text-sm font-bold tracking-tight text-[var(--text-primary)] leading-tight">kfont</h1>
        <p class="text-[11px] text-[var(--text-muted)] leading-none">Font Viewer</p>
      </div>
    </div>

    <button
      type="button"
      onclick={onOpenSettings}
      class="p-1.5 rounded-md text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
      title="Settings and Themes"
      aria-label="Settings and Themes"
    >
      <Settings size={17} />
    </button>
  </div>

  <!-- Folder & Sources Section -->
  <div class="flex-1 overflow-y-auto px-3 py-3 space-y-4">
    <!-- Sources Section -->
    <div>
      <div class="px-2 mb-1.5 flex items-center justify-between">
        <span class="text-[11px] font-semibold uppercase tracking-wider text-[var(--text-muted)]">Folders</span>
        <div class="flex items-center gap-1">
          <button
            type="button"
            onclick={onAddFolder}
            class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-card)] transition-colors focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
            title="Add local folder"
            aria-label="Add local folder"
          >
            <FolderPlus size={15} />
          </button>
          <button
            type="button"
            onclick={onAddFiles}
            class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-card)] transition-colors focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
            title="Open specific font files"
            aria-label="Open specific font files"
          >
            <FilePlus size={15} />
          </button>
        </div>
      </div>

      <nav class="space-y-0.5" aria-label="Font Folders">
        {#each folders as folder}
          <div class="group flex items-center gap-1 rounded-md transition-colors {selectedFolderId === folder.id ? 'bg-[var(--bg-card-active)] text-[var(--text-primary)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}">
            <button
              type="button"
              onclick={() => onSelectFolder(folder.id)}
              class="flex-1 flex items-center justify-between px-2.5 py-1.5 text-xs text-left truncate focus-visible:ring-2 focus-visible:ring-[var(--accent)] rounded-md outline-none"
            >
              <div class="flex items-center gap-2 truncate">
                <Folder size={14} class={selectedFolderId === folder.id ? 'text-[var(--accent)]' : 'text-[var(--text-muted)]'} />
                <span class="truncate" title={folder.path}>{folder.name}</span>
              </div>
              <span class="text-[11px] text-[var(--text-muted)] tabular-nums ml-1">
                {folder.count}
              </span>
            </button>

            {#if folder.id !== 'system'}
              <button
                type="button"
                onclick={(e) => {
                  e.stopPropagation();
                  onRemoveFolder(folder.id);
                }}
                class="opacity-0 group-hover:opacity-100 p-1 mr-1 rounded text-[var(--text-muted)] hover:text-red-400 hover:bg-[var(--bg-card)] transition-opacity focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
                title="Remove folder from list"
                aria-label="Remove folder"
              >
                <Trash2 size={13} />
              </button>
            {/if}
          </div>
        {/each}
      </nav>
    </div>

    <!-- Category Filters -->
    <div>
      <div class="px-2 mb-1.5 flex items-center justify-between">
        <span class="text-[11px] font-semibold uppercase tracking-wider text-[var(--text-muted)]">Classification</span>
      </div>

      <nav class="space-y-0.5" aria-label="Classification Filters">
        {#each categories as cat}
          <button
            type="button"
            onclick={() => onSelectCategory(cat.id)}
            class="w-full flex items-center justify-between px-2.5 py-1.5 text-xs rounded-md transition-colors text-left focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none {filterCategory === cat.id ? 'bg-[var(--bg-card-active)] text-[var(--text-primary)] font-medium' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-card-hover)]'}"
          >
            <div class="flex items-center gap-2">
              <Type size={14} class={filterCategory === cat.id ? 'text-[var(--accent)]' : 'text-[var(--text-muted)]'} />
              <span>{cat.label}</span>
            </div>
          </button>
        {/each}
      </nav>
    </div>
  </div>

  <!-- Footer Stats & Actions -->
  <div class="p-3 border-t border-[var(--border-subtle)] bg-[var(--bg-sidebar)] flex items-center justify-between text-xs text-[var(--text-muted)]">
    <div class="flex flex-col">
      <span class="font-medium text-[var(--text-primary)] tabular-nums">{filteredCount} fonts</span>
      {#if filteredCount !== totalFontsCount}
        <span class="text-[10px] text-[var(--text-muted)]">of {totalFontsCount} total</span>
      {/if}
    </div>

    <button
      type="button"
      onclick={onRefresh}
      disabled={isScanning}
      class="p-1.5 rounded-md hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors disabled:opacity-50 focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none"
      title="Rescan font directories"
      aria-label="Rescan font directories"
    >
      <RefreshCw size={15} class={isScanning ? 'animate-spin text-[var(--accent)]' : ''} />
    </button>
  </div>
</aside>
