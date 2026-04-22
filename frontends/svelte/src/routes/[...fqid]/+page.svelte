<script lang="ts">
	import { page } from '$app/state';
	import symbolTableJson from '$lib/json/symbol_table.json';
	import type { DoqoSymbolTable } from '$lib/bindings/DoqoSymbolTable';
	import { pathToFqid, symbolName, source } from '$lib/utils';
	import SymbolCard from '$lib/components/SymbolCard.svelte';
	import SymbolBreadcrumbs from '$lib/components/SymbolBreadcrumbs.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Documentation from '$lib/components/Documentation.svelte';

	const symbolTable = symbolTableJson as DoqoSymbolTable;
	const languages = $derived([
		...new Set(Object.values(symbolTable.sources).map((s) => s.language))
	]);

	const path = $derived(page.params.fqid ?? '');
	const fqid = $derived(pathToFqid(path));

	const symbol = $derived(Object.values(symbolTable.symbols).find((s) => s.fqid === fqid));
	const language = $derived(symbol ? symbol.language : '');
	const code = $derived(symbol ? source(symbol, symbolTable) : '');

	const childrenList = $derived(
		symbol?.children.map((id) => symbolTable.symbols[id]).filter(Boolean) ?? []
	);
	const groupedChildren = $derived(Object.groupBy(childrenList, (c) => c.kind));
	const childKinds = $derived(Object.keys(groupedChildren).sort());
</script>

{#if symbol}
	<div class="mx-auto max-w-5xl p-8">
		<header class="mb-10">
			<SymbolBreadcrumbs {symbol} />

			<div class="mt-4 flex items-center justify-between gap-4">
				<h1 class="font-mono text-4xl font-bold tracking-tight text-slate-900">
					{symbolName(symbol)}
				</h1>

				<span
					class="rounded-full border border-slate-100 bg-slate-50 px-3 py-1 text-xs font-semibold tracking-wider text-slate-700 uppercase shadow-sm"
				>
					{symbol.kind}
				</span>
			</div>
		</header>

		<div class="flex flex-col gap-12">
			<section class="space-y-4">
				<div class="border-b border-slate-100 pb-2">
					<h2 class="text-xs font-bold tracking-widest text-slate-400 uppercase">Documentation</h2>
				</div>
				<div class="prose-custom">
					<Documentation {symbol} />
				</div>
			</section>

			<section class="space-y-4">
				<div class="border-b border-slate-100 pb-2">
					<h2 class="text-xs font-bold tracking-widest text-slate-400 uppercase">Source code</h2>
				</div>
				<CodeBlock {code} {language} {languages} />
			</section>

			{#if childKinds.length > 0}
				<section class="space-y-8">
					<div class="border-b border-slate-100 pb-2">
						<h2 class="text-xs font-bold tracking-widest text-slate-400 uppercase">Members</h2>
					</div>

					{#each childKinds as kind}
						<div class="space-y-4">
							<h3 class="text-sm font-semibold text-slate-600 capitalize">
								{kind}
							</h3>

							<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
								{#each groupedChildren[kind] as child}
									<SymbolCard symbol={child} />
								{/each}
							</div>
						</div>
					{/each}
				</section>
			{/if}
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center p-20 text-center">
		<h1 class="mb-4 text-2xl font-bold text-slate-800">Symbol not found</h1>
		<p class="mb-6 font-mono text-slate-500">{fqid}</p>
		<a
			href="/"
			class="rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-slate-800"
		>
			Return to index
		</a>
	</div>
{/if}
