<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { Settings, User, ChevronRight } from 'lucide-svelte';

	let user = $state<any>(null);
	let loading = $state(true);

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') return localStorage.getItem('forge_token');
		return null;
	}

	onMount(async () => {
		const token = getToken();
		if (token) {
			try {
				const res = await fetch(`${API}/users/me`, {
					headers: { Authorization: `Bearer ${token}` }
				});
				if (res.ok) user = await res.json();
			} catch {}
		}
		loading = false;
	});
</script>

<div class="h-full flex flex-col">
	<header class="flex items-center gap-2 px-6 h-12 border-b border-surface-800/30 text-xs text-surface-500 shrink-0">
		<a href="/" class="hover:text-surface-300 transition-colors">Dashboard</a>
		<ChevronRight size={12} />
		<span class="text-surface-400">Settings</span>
	</header>

	<div class="flex-1 overflow-y-auto p-5">
		<div class="max-w-lg" in:fade={{ duration: 300 }}>
			<h2 class="text-base font-medium text-surface-100 mb-4">Profile</h2>
			{#if loading}
				<div class="space-y-3 animate-pulse">
					<div class="h-4 bg-surface-800/30 rounded w-1/3" />
					<div class="h-4 bg-surface-800/20 rounded w-1/2" />
					<div class="h-4 bg-surface-800/20 rounded w-2/3" />
				</div>
			{:else if user}
				<div class="bg-surface-800/10 border border-surface-800/30 rounded-xl p-5">
					<div class="flex items-center gap-3 mb-5">
						<div class="w-10 h-10 rounded-full bg-gradient-to-br from-forge-400 to-forge-600 flex items-center justify-center text-sm font-bold text-white">
							{user.username[0].toUpperCase()}
						</div>
						<div>
							<h3 class="text-sm font-medium text-surface-200">{user.username}</h3>
							<p class="text-xs text-surface-500">{user.email}</p>
						</div>
					</div>
					<div class="space-y-0 divide-y divide-surface-800/20 text-sm">
						<div class="flex items-center justify-between py-3">
							<span class="text-surface-400">Username</span>
							<span class="text-surface-200">{user.username}</span>
						</div>
						<div class="flex items-center justify-between py-3">
							<span class="text-surface-400">Email</span>
							<span class="text-surface-200">{user.email}</span>
						</div>
						<div class="flex items-center justify-between py-3">
							<span class="text-surface-400">Repositories</span>
							<span class="text-surface-200">{user.repo_count}</span>
						</div>
					</div>
				</div>
			{:else}
				<p class="text-sm text-surface-500">Not authenticated</p>
			{/if}
		</div>
	</div>
</div>