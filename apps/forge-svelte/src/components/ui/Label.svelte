<script lang="ts">
	export let style: string = '';
</script>

<label class = ${"text-xs ml-1 font-medium text-white/60" + style}>
	<slot />
</label>
