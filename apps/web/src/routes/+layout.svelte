<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';

	let { children } = $props();

	let mounted = $state(false);

	onMount(() => {
		mounted = true;
	});
</script>

{#if mounted}
	<div class="min-h-screen bg-surface-950 text-surface-100 transition-all duration-300">
		{@render children()}
	</div>
{/if}