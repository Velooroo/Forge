<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { GitBranch, Plus, LogOut, FileCode2, Search } from 'lucide-svelte';

	let { children } = $props();

	let user = $state<any>(null);
	let searchQuery = $state('');
	let mounted = $state(false);

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') return localStorage.getItem('forge_token');
		return null;
	}

	async function fetchUser() {
		const token = getToken();
		if (!token) return;
		try {
			const res = await fetch(`${API}/users/me`, {
				headers: { Authorization: `Bearer ${token}` }
			});
			if (res.ok) user = await res.json();
		} catch {}
	}

	function logout() {
		localStorage.removeItem('forge_token');
		window.location.href = '/auth';
	}

	onMount(() => { mounted = true; fetchUser(); });

	let isAuthPage = $derived($page.url.pathname === '/auth');
</script>

{#if mounted}
	{#if isAuthPage}
		{@render children()}
	{:else}
		<div class="flex h-screen overflow-hidden bg-surface-950">
			<!-- Sidebar — full height, no rounding -->
			<aside class="w-60 lg:w-64 bg-surface-950 border-r border-surface-800/30 flex flex-col shrink-0">
				<!-- Logo area -->
				<div class="flex items-center gap-3 px-5 h-14 border-b border-surface-800/20">
					<div class="w-7 h-7 rounded-lg bg-gradient-to-br from-forge-400 to-forge-600 flex items-center justify-center">
						<FileCode2 size={14} class="text-white" />
					</div>
					<div>
						<h1 class="font-semibold text-surface-100 text-sm tracking-tight">Forge</h1>
						<p class="text-[10px] text-surface-500 -mt-0.5">by Veloro</p>
					</div>
				</div>

				<!-- Search -->
				<div class="px-3 pt-3 pb-2">
					<div class="relative">
						<Search size={14} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-surface-500 pointer-events-none" />
						<input
							type="text"
							placeholder="Search repos..."
							bind:value={searchQuery}
							class="w-full pl-8 pr-3 py-1.5 bg-surface-900/50 border border-surface-800/50 rounded-lg text-xs text-surface-300 
								   placeholder:text-surface-600 focus:outline-none focus:border-forge-500/50 focus:ring-1 focus:ring-forge-500/20 transition-all"
						/>
					</div>
				</div>

				<!-- Nav -->
<nav class="flex-1 overflow-y-auto px-3 py-1 space-y-0.5">
					<a
						href="/"
						class="flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm transition-all duration-150 {$page.url.pathname === '/' ? 'bg-surface-800/40 text-surface-200' : 'text-surface-400 hover:bg-surface-800/20 hover:text-surface-200'}"
					>
						<GitBranch size={15} class="shrink-0" />
						<span>Repositories</span>
					</a>

					<!-- User info -->
					{#if user}
						<div class="flex items-center gap-2.5 px-3 py-2 mt-2 text-xs text-surface-500 border-t border-surface-800/20 pt-3">
							<div class="w-5 h-5 rounded-full bg-forge-500/20 flex items-center justify-center text-[9px] font-bold text-forge-400 shrink-0">
								{user.username[0].toUpperCase()}
							</div>
							<span class="truncate">{user.username}</span>
						</div>
					{/if}

					<button
						onclick={logout}
						class="flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-sm text-surface-500 
							   hover:bg-red-500/5 hover:text-red-400 transition-all duration-150"
					>
						<LogOut size={15} />
						<span>Logout</span>
					</button>
				</nav>
			</aside>

			<!-- Main area — rounded card with margins -->
			<main class="flex-1 flex flex-col min-w-0 overflow-hidden">
				<div class="flex-1 m-3 bg-surface-900/40 backdrop-blur-sm border border-surface-800/30 rounded-2xl overflow-hidden flex flex-col">
					{@render children()}
				</div>
			</main>
		</div>
	{/if}
{/if}