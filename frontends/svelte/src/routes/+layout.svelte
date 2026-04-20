<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
  import symbolTable from '$lib/symbol_table.json'

	let { children } = $props();
  let symbols = Object.values(symbolTable.symbols);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex h-screen">
  <aside class="border-r border-black overflow-y-auto px-2">
    <nav>
      <h2>Symbols</h2>
      <ul>
        {#each symbols as symbol}
          <li>{symbol.fqid}</li>
        {/each}
      </ul>
    </nav>
  </aside>

  <div class="flex-1 overflow-y-auto px-2">
    {@render children()}
  </div>
</div>
