<script lang="ts">
	import { LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, User, Key } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade, slide } from 'svelte/transition';
	import { cubicOut, quintOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();

	let activePanel = $state<'none' | 'nav' | 'files' | 'settings'>('none');
	let mounted = $state(false);

	onMount(() => { mounted = true; });

	function togglePanel(panel: 'nav' | 'files' | 'settings') {
		activePanel = activePanel === panel ? 'none' : panel;
	}
</script>

<div class="flex min-h-screen bg-[#08090b] text-white">
	<!-- ACTIVITY RAIL -->
	<aside class="flex w-[52px] shrink-0 flex-col items-center border-r border-white/[0.06] bg-[#0c0d0f] py-3">
		<button
			onclick={() => window.location.href = '/dashboard'}
			class="mb-4 flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-red-500 to-rose-600 shadow-lg shadow-red-500/20"
		>
			<Cpu class="h-4 w-4 text-white" />
		</button>

		<div class="flex flex-col items-center gap-1">
			<button
				onclick={() => togglePanel('nav')}
				class="flex h-9 w-9 items-center justify-center rounded-lg transition-all duration-150 {activePanel === 'nav' ? 'bg-red-500/10 text-red-400' : 'bg-white/[0.04] text-white/40 hover:bg-white/10 hover:text-white'}"
			>
				<LayoutDashboard class="h-[18px] w-[18px]" />
			</button>
			<button
				onclick={() => togglePanel('files')}
				class="flex h-9 w-9 items-center justify-center rounded-lg transition-all duration-150 {activePanel === 'files' ? 'bg-red-500/10 text-red-400' : 'bg-white/[0.04] text-white/40 hover:bg-white/10 hover:text-white'}"
			>
				<FolderGit2 class="h-[18px] w-[18px]" />
			</button>
		</div>

		<div class="mt-auto flex flex-col items-center gap-1">
			<button
				onclick={() => togglePanel('settings')}
				class="flex h-9 w-9 items-center justify-center rounded-lg transition-all duration-150 {activePanel === 'settings' ? 'bg-red-500/10 text-red-400' : 'bg-white/[0.04] text-white/40 hover:bg-white/10 hover:text-white'}"
			>
				<Settings class="h-[18px] w-[18px]" />
			</button>
			<button
				onclick={logout}
				class="flex h-9 w-9 items-center justify-center rounded-lg text-white/30 transition-all hover:bg-white/10 hover:text-red-400"
			>
				<LogOut class="h-[18px] w-[18px]" />
			</button>
		</div>
	</aside>

	<!-- OVERLAY SIDEBAR -->
	{#if activePanel !== 'none'}
		<div
			class="w-72 shrink-0 border-r border-white/[0.06] bg-[#0c0d0f] overflow-y-auto"
			transition:slide={{ duration: 200, easing: quintOut, axis: 'x' }}
		>
			{#if activePanel === 'nav'}
				<nav class="p-3 space-y-0.5">
					<div class="mb-2 px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.12em] text-white/30">Navigate</div>
					<a href="/dashboard"
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/60 transition-all hover:bg-white/[0.06] hover:text-white"
					>
						<LayoutDashboard class="h-4 w-4" />
						Dashboard
					</a>
					<a href="/dashboard"
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/60 transition-all hover:bg-white/[0.06] hover:text-white"
					>
						<FolderGit2 class="h-4 w-4" />
						Repositories
					</a>
					<a href="/dashboard/settings"
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/60 transition-all hover:bg-white/[0.06] hover:text-white"
					>
						<Settings class="h-4 w-4" />
						Settings
					</a>
				</nav>
			{:else if activePanel === 'files'}
				<div class="p-3">
					<div class="mb-2 px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.12em] text-white/30">Files</div>
					<p class="px-3 text-xs text-white/20">Open a repository to browse files</p>
				</div>
			{:else if activePanel === 'settings'}
				<nav class="p-3 space-y-0.5">
					<div class="mb-2 px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.12em] text-white/30">Settings</div>
					<a href="/dashboard/settings"
						class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/60 transition-all hover:bg-white/[0.06] hover:text-white"
					>
						<User class="h-4 w-4" />
						Account
					</a>
					<button disabled
						class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/30 transition-all cursor-not-allowed"
					>
						<Key class="h-4 w-4" />
						Access Tokens
						<span class="ml-auto text-[10px] text-white/20">soon</span>
					</button>
				</nav>
			{/if}
		</div>
	{/if}

	<!-- MAIN -->
	<main class="flex-1 overflow-auto">
		{#if mounted}
			<div in:fade={{ duration: 200, easing: cubicOut }}>
				{@render children()}
			</div>
		{/if}
	</main>
</div>
