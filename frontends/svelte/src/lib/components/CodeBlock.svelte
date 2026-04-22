<script lang="ts">
  import { getHighlighter } from '$lib/highlighter.svelte';

  let { code, language, languages } = $props<{ 
    code: string; 
    language: string;
    languages: string[];
  }>();

  let highlightedHtml = $state<string>('');
  let isLoading = $state(true);

  $effect(() => {
    if (!code) return;

    const highlight = async () => {
      isLoading = true;
      
      const highlighter = await getHighlighter(languages);
      
      const lang = language.toLowerCase();

      try {
        await highlighter.loadLanguage(lang as any);
        
        highlightedHtml = highlighter.codeToHtml(code.trim(), {
          lang,
          theme: 'github-dark'
        });
      } catch (e) {
        console.warn(`Shiki error: ${lang}`, e);
        highlightedHtml = `<pre class="shiki"><code>${code}</code></pre>`;
      } finally {
        isLoading = false;
      }
    };

    highlight();
  });
</script>

<div class="relative group rounded-lg border border-slate-800 bg-[#0d1117] font-mono text-sm overflow-hidden">
    <div class="absolute right-3 top-2 text-[10px] font-bold uppercase text-slate-500 select-none z-10">
      {language}
    </div>
  
    <div class="shiki-container">
      {@html highlightedHtml}
    </div>
</div>

<style>
  :global(.shiki-container pre) {
    margin: 0 !important;
    padding: 1rem !important;
    background-color: transparent !important;
    overflow-x: auto;
    line-height: 1.6;
  }

  :global(.shiki-container code) {
    background: none !important;
    padding: 0 !important;
  }
</style>