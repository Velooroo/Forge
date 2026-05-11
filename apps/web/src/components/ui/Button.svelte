<script lang="ts">
	let { type = 'button', variant = 'default', class_el = '', onclick, children }: {
		type?: 'button' | 'submit';
		variant?: 'default' | 'outline';
		class_el?: string;
		onclick?: (e: MouseEvent) => void;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<button
	{type}
	{onclick}
	class={`${variant === 'outline'
		? 'border border-white/10 bg-white/5 text-white/80 hover:bg-white/10 hover:text-white'
		: 'h-12 w-full rounded-2xl bg-gradient-to-r from-red-500 to-rose-600 font-semibold text-white/80 shadow-lg shadow-red-500/15 transition-all hover:scale-[1.015] hover:shadow-red-500/25'} ${class_el}`}
>
	{#if children}{@render children()}{/if}
</button>
