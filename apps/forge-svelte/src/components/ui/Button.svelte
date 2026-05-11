<script lang="ts">
	export let type: 'button' | 'submit' = 'button';
	export let variant: 'default' | 'outline' = 'default';
	export let class_el: string = '';
</script>

<button
	{type}
	class={`${
		variant === 'outline'
			? 'border border-white/10 bg-white/5 text-white/80 hover:bg-white/10 hover:text-white'
			: ''
	}
	"h-12 hover:shadow-emerald-500/25"" w-full rounded-2xl bg-gradient-to-r from-emerald-500
    to-cyan-500 font-semibold text-white/80 shadow-lg
     shadow-emerald-500/15 transition-all hover:scale-[1.015]`}
>
	<slot />
</button>
