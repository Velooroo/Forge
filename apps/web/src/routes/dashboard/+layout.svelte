<script lang="ts">
	import { Github, LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, ChevronLeft, ChevronRight } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();

	let collapsed = $state(false);
	let mounted = $state(false);

	onMount(() => { mounted = true; });

	function goHome() {
		window.location.href = '/dashboard';
	}
</script>

<div class="flex min-h-screen bg-[#030607] text-white">
	<!-- SIDEBAR -->
	<aside
		class="relative flex flex-col border-r border-white/10 bg-[#0a0e10] transition-all duration-300"
		style="width: {collapsed ? '64px' : '240px'}"
	>
		<div class="flex h-16 items-center justify-between border-b border-white/10 px-4">
			{#if !collapsed}
				<button onclick={goHome} class="flex items-center gap-2 font-semibold text-white/80 hover:text-white">
					<Cpu class="h-5 w-5 text-emerald-400" />
					<span>Forge</span>
				</button>
			{/if}
			<button
				onclick={() => collapsed = !collapsed}
				class="ml-auto flex h-8 w-8 items-center justify-center rounded-lg text-white/40 hover:bg-white/10 hover:text-white"
			>
				{#if collapsed}
					<ChevronRight class="h-4 w-4" />
				{:else}
					<ChevronLeft class="h-4 w-4" />
				{/if}
			</button>
		</div>

		<nav class="flex-1 space-y-1 p-3">
			<a
				href="/dashboard"
				class="flex items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/60 transition-all hover:bg-white/5 hover:text-white"
			>
				<LayoutDashboard class="h-4 w-4 shrink-0" />
				{#if !collapsed}
					<span>Dashboard</span>
				{/if}
			</a>
			<a
				href="/dashboard/repos"
				class="flex items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/60 transition-all hover:bg-white/5 hover:text-white"
			>
				<FolderGit2 class="h-4 w-4 shrink-0" />
				{#if !collapsed}
					<span>Repositories</span>
				{/if}
			</a>
			<a
				href="/dashboard/settings"
				class="flex items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/60 transition-all hover:bg-white/5 hover:text-white"
			>
				<Settings class="h-4 w-4 shrink-0" />
				{#if !collapsed}
					<span>Settings</span>
				{/if}
			</a>
		</nav>

		<div class="border-t border-white/10 p-3">
			<button
				onclick={logout}
				class="flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/40 transition-all hover:bg-white/5 hover:text-red-400"
			>
				<LogOut class="h-4 w-4 shrink-0" />
				{#if !collapsed}
					<span>Sign Out</span>
				{/if}
			</button>
		</div>
	</aside>

	<!-- MAIN -->
	<main class="flex-1 overflow-auto">
		{#if mounted}
			<div transition:fade={{ duration: 300, easing: cubicOut }}>
				{@render children()}
			</div>
		{/if}
	</main>
</div>
