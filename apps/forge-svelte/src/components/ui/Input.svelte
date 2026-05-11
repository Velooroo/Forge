<script lang="ts">
	export let type: string = 'text';
	export let placeholder: string = '';
	export let class_el: string = '';
	export let value: string = '';
</script>

<input
	{type}
	{placeholder}
	class={class_el + `
	h-14 rounded-2xl
	border-white/10 bg-white/5 text-white 
	transition-all placeholder:text-white/25 
	focus:border-emerald-400/40 focus:bg-white/10 focus:ring-emerald-500/20`}
	bind:value
/>
