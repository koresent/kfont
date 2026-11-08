import { invoke } from '@tauri-apps/api/core';

// Strict memory cap: Keep at most 160 active FontFace objects in WebKit memory.
// When the threshold is reached, the least recently used fonts are evicted via document.fonts.delete.
const MAX_LOADED_FONTS = 160;
const loadedFontFaces = new Map<string, FontFace>();
const loadingPromises = new Map<string, Promise<string>>();

export function isFontLoaded(fontId: string): boolean {
  return loadedFontFaces.has(fontId);
}

export function getCustomFamilyName(fontId: string): string {
  return `kfont_${fontId.replace(/[^a-zA-Z0-9]/g, '_')}`;
}

export async function ensureFontLoaded(filePath: string, fontId: string): Promise<string> {
  const customFamily = getCustomFamilyName(fontId);

  // LRU cache hit: move to most recent position
  if (loadedFontFaces.has(fontId)) {
    const existing = loadedFontFaces.get(fontId)!;
    loadedFontFaces.delete(fontId);
    loadedFontFaces.set(fontId, existing);
    return customFamily;
  }

  if (loadingPromises.has(fontId)) {
    return loadingPromises.get(fontId)!;
  }

  const loadPromise = (async () => {
    try {
      let fontFace: FontFace;

      // 1. Try streaming raw bytes via asynchronous kfont:// protocol
      try {
        const url = `kfont://localhost${encodeURI(filePath)}`;
        fontFace = new FontFace(customFamily, `url("${url}")`);
        await fontFace.load();
      } catch {
        // 2. Fallback to direct binary IPC ArrayBuffer
        const buffer = await invoke<ArrayBuffer>('get_font_bytes', { path: filePath });
        fontFace = new FontFace(customFamily, buffer);
        await fontFace.load();
      }

      // Evict oldest FontFace if cap exceeded to prevent unbounded memory growth
      if (loadedFontFaces.size >= MAX_LOADED_FONTS) {
        const oldestKey = loadedFontFaces.keys().next().value;
        if (oldestKey) {
          const oldFace = loadedFontFaces.get(oldestKey);
          if (oldFace) {
            document.fonts.delete(oldFace);
          }
          loadedFontFaces.delete(oldestKey);
        }
      }

      document.fonts.add(fontFace);
      loadedFontFaces.set(fontId, fontFace);
      return customFamily;
    } catch (err) {
      console.error('Failed to load font:', filePath, err);
      return 'sans-serif';
    } finally {
      loadingPromises.delete(fontId);
    }
  })();

  loadingPromises.set(fontId, loadPromise);
  return loadPromise;
}
