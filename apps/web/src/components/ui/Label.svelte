<script lang="ts">
	let { style = '', children }: {
		style?: string;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<label class={"text-xs ml-1 font-medium text-white/60" + style}>
	{#if children}{@render children()}{/if}
</label>
