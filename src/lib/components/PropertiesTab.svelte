<script lang="ts">
  import { Folder, ExternalLink, Info, Shield, Cpu, HardDrive } from '@lucide/svelte';
  import type { FontSummary, FontDetails } from '$lib/types';
  import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener';

  let {
    font,
    details,
  }: {
    font: FontSummary;
    details: FontDetails | null;
  } = $props();

  async function handleRevealFile() {
    try {
      await revealItemInDir(font.file_path);
    } catch (e) {
      console.error('Failed to reveal file:', e);
    }
  }

  async function handleOpenUrl(url: string | null | undefined) {
    if (!url) return;
    try {
      await openUrl(url);
    } catch (e) {
      console.error('Failed to open URL:', e);
    }
  }
</script>

<div class="h-full overflow-y-auto p-6 space-y-6 select-text">
  <!-- Header Card with File Actions -->
  <div class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] flex flex-wrap items-center justify-between gap-4">
    <div class="space-y-1">
      <h2 class="text-base font-bold text-[var(--text-primary)]">{font.full_name}</h2>
      <p class="text-xs text-[var(--text-muted)] font-mono break-all">{font.file_path}</p>
    </div>

    <button
      type="button"
      onclick={handleRevealFile}
      class="px-3 py-1.5 rounded-md bg-[var(--accent)] text-[var(--accent-text)] font-semibold text-xs flex items-center gap-2 hover:opacity-90 transition-opacity focus-visible:ring-2 focus-visible:ring-[var(--accent)] outline-none cursor-pointer"
    >
      <Folder size={15} />
      <span>Show in File Manager</span>
    </button>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Font Identification -->
    <div class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-4">
      <div class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-[var(--accent)]">
        <Info size={15} />
        <span>Identification</span>
      </div>

      <dl class="space-y-2.5 text-xs">
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Font Family:</dt>
          <dd class="font-medium text-[var(--text-primary)] text-right">{font.family}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Subfamily / Style:</dt>
          <dd class="font-medium text-[var(--text-primary)] text-right">{font.style}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Full Font Name:</dt>
          <dd class="font-medium text-[var(--text-primary)] text-right">{font.full_name}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">PostScript Name:</dt>
          <dd class="font-mono text-[var(--text-primary)] text-right">{font.postscript_name}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Version:</dt>
          <dd class="font-mono text-[var(--text-primary)] text-right">{details?.version || 'N/A'}</dd>
        </div>
      </dl>
    </div>

    <!-- Metrics & Technical Specs -->
    <div class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-4">
      <div class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-[var(--accent)]">
        <Cpu size={15} />
        <span>Technical Metrics</span>
      </div>

      <dl class="space-y-2.5 text-xs">
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Format:</dt>
          <dd class="font-semibold text-[var(--text-primary)]">{font.format}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Glyph Count:</dt>
          <dd class="font-mono text-[var(--text-primary)]">{font.glyph_count.toLocaleString()}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Units Per Em:</dt>
          <dd class="font-mono text-[var(--text-primary)]">{details?.units_per_em || 'N/A'}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Weight Class:</dt>
          <dd class="font-mono text-[var(--text-primary)]">{font.weight}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Width Class:</dt>
          <dd class="text-[var(--text-primary)]">{font.width}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Monospaced:</dt>
          <dd class="text-[var(--text-primary)]">{font.is_monospaced ? 'Yes' : 'No'}</dd>
        </div>
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Ascender / Descender:</dt>
          <dd class="font-mono text-[var(--text-primary)]">{details?.ascender} / {details?.descender}</dd>
        </div>
      </dl>
    </div>

    <!-- Designer & Attribution -->
    <div class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-4">
      <div class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-[var(--accent)]">
        <Info size={15} />
        <span>Designer & Foundry</span>
      </div>

      <dl class="space-y-2.5 text-xs">
        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Designer:</dt>
          <dd class="font-medium text-[var(--text-primary)] text-right">{details?.designer || 'Unspecified'}</dd>
        </div>

        {#if details?.designer_url}
          <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
            <dt class="text-[var(--text-muted)]">Designer Web:</dt>
            <dd>
              <button
                type="button"
                onclick={() => handleOpenUrl(details?.designer_url)}
                class="text-[var(--accent)] hover:underline flex items-center gap-1 cursor-pointer"
              >
                <span class="max-w-[200px] truncate">{details.designer_url}</span>
                <ExternalLink size={12} />
              </button>
            </dd>
          </div>
        {/if}

        <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
          <dt class="text-[var(--text-muted)]">Manufacturer / Foundry:</dt>
          <dd class="font-medium text-[var(--text-primary)] text-right">{details?.manufacturer || 'Unspecified'}</dd>
        </div>

        {#if details?.vendor_url}
          <div class="flex items-start justify-between border-b border-[var(--border-subtle)] pb-2">
            <dt class="text-[var(--text-muted)]">Vendor Web:</dt>
            <dd>
              <button
                type="button"
                onclick={() => handleOpenUrl(details?.vendor_url)}
                class="text-[var(--accent)] hover:underline flex items-center gap-1 cursor-pointer"
              >
                <span class="max-w-[200px] truncate">{details.vendor_url}</span>
                <ExternalLink size={12} />
              </button>
            </dd>
          </div>
        {/if}
      </dl>
    </div>

    <!-- Legal & License -->
    <div class="p-5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-4">
      <div class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-[var(--accent)]">
        <Shield size={15} />
        <span>Legal & License</span>
      </div>

      <dl class="space-y-2.5 text-xs">
        <div class="border-b border-[var(--border-subtle)] pb-2 space-y-1">
          <dt class="text-[var(--text-muted)]">Copyright Notice:</dt>
          <dd class="text-[var(--text-primary)] leading-relaxed">{details?.copyright || 'No copyright notice provided'}</dd>
        </div>

        {#if details?.trademark}
          <div class="border-b border-[var(--border-subtle)] pb-2 space-y-1">
            <dt class="text-[var(--text-muted)]">Trademark:</dt>
            <dd class="text-[var(--text-primary)]">{details.trademark}</dd>
          </div>
        {/if}

        {#if details?.license}
          <div class="border-b border-[var(--border-subtle)] pb-2 space-y-1">
            <dt class="text-[var(--text-muted)]">License Description:</dt>
            <dd class="text-[var(--text-secondary)] leading-relaxed max-h-24 overflow-y-auto">{details.license}</dd>
          </div>
        {/if}

        {#if details?.license_url}
          <div class="flex items-start justify-between pt-1">
            <dt class="text-[var(--text-muted)]">License URL:</dt>
            <dd>
              <button
                type="button"
                onclick={() => handleOpenUrl(details?.license_url)}
                class="text-[var(--accent)] hover:underline flex items-center gap-1 cursor-pointer"
              >
                <span class="max-w-[200px] truncate">{details.license_url}</span>
                <ExternalLink size={12} />
              </button>
            </dd>
          </div>
        {/if}
      </dl>
    </div>
  </div>
</div>
