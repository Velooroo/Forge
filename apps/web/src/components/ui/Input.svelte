<script lang="ts">
	let { type = 'text', placeholder = '', class_el = '', value = $bindable(''), oninput }: {
		type?: string;
		placeholder?: string;
		class_el?: string;
		value?: string;
		oninput?: (e: Event) => void;
	} = $props();
</script>

<input
	{type}
	{placeholder}
	bind:value
	{oninput}
	class={class_el + ' h-14 rounded-2xl border-white/10 bg-white/5 text-white transition-all placeholder:text-white/25 focus:border-red-400/40 focus:bg-white/10 focus:ring-red-500/20'}
/>
