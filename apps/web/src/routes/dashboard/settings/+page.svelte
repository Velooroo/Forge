<script lang="ts">
	import { onMount } from 'svelte';
	import { Settings, User, Shield, Bell, ChevronRight } from 'lucide-svelte';

	let user = $state<any>(null);
	let loading = $state(true);

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') {
			return localStorage.getItem('forge_token');
		}
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

<div class="min-h-screen bg-surface-950">
	<header class="border-b border-surface-800/50 glass">
		<div class="max-w-4xl mx-auto px-6">
			<div class="flex items-center gap-2 h-12 text-sm text-surface-400">
				<a href="/" class="hover:text-surface-200 transition-colors">Dashboard</a>
				<ChevronRight size={14} />
				<span class="text-surface-100">Settings</span>
			</div>
			<h1 class="text-xl font-bold text-surface-100 pb-4">Settings</h1>
		</div>
	</header>

	<div class="max-w-4xl mx-auto px-6 py-6">
		<div class="glass rounded-xl p-6 animate-fade-in">
			{#if loading}
				<div class="animate-pulse space-y-3">
					<div class="h-4 bg-surface-800 rounded w-1/3" />
					<div class="h-4 bg-surface-800 rounded w-1/2" />
				</div>
			{:else if user}
				<div class="flex items-center gap-4 mb-6">
					<div class="w-14 h-14 rounded-full bg-gradient-to-br from-forge-400 to-forge-600 flex items-center justify-center text-xl font-medium text-white">
						{user.username[0].toUpperCase()}
					</div>
					<div>
						<h2 class="text-lg font-medium text-surface-100">{user.username}</h2>
						<p class="text-sm text-surface-400">{user.email}</p>
					</div>
				</div>

				<div class="space-y-4">
					<div class="flex items-center justify-between py-3 border-b border-surface-800/50">
						<div>
							<p class="text-sm text-surface-200">Username</p>
							<p class="text-xs text-surface-400">{user.username}</p>
						</div>
					</div>
					<div class="flex items-center justify-between py-3 border-b border-surface-800/50">
						<div>
							<p class="text-sm text-surface-200">Email</p>
							<p class="text-xs text-surface-400">{user.email}</p>
						</div>
					</div>
					<div class="flex items-center justify-between py-3">
						<div>
							<p class="text-sm text-surface-200">Repositories</p>
							<p class="text-xs text-surface-400">{user.repo_count} repos</p>
						</div>
					</div>
				</div>
			{:else}
				<p class="text-surface-400 text-sm">Not authenticated</p>
			{/if}
		</div>
	</div>
</div>