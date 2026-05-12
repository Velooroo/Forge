<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { GitBranch, Plus, Settings, LogOut, FileCode2, ChevronRight, Github } from 'lucide-svelte';

	let { data } = $props();

	let loading = $state(false);
	let repos: any[] = $state([]);
	let user = $state<any>(null);
	let showCreateModal = $state(false);
	let newRepoName = $state('');
	let newRepoDesc = $state('');
	let newRepoPrivate = $state(false);
	let error = $state('');

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') {
			return localStorage.getItem('forge_token');
		}
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

	async function fetchRepos() {
		loading = true;
		try {
			const token = getToken();
			const res = await fetch(`${API}/repos/mine`, {
				headers: token ? { Authorization: `Bearer ${token}` } : {}
			});
			if (res.ok) repos = await res.json();
		} catch {}
		loading = false;
	}

	async function createRepo() {
		if (!newRepoName.trim()) return;
		error = '';
		const token = getToken();
		if (!token) { error = 'Not authenticated'; return; }
		try {
			const res = await fetch(`${API}/repos/create`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					name: newRepoName.trim(),
					description: newRepoDesc.trim() || null,
					is_private: newRepoPrivate
				})
			});
			if (res.ok) {
				showCreateModal = false;
				newRepoName = '';
				newRepoDesc = '';
				newRepoPrivate = false;
				await fetchRepos();
			} else {
				const err = await res.text();
				error = err;
			}
		} catch { error = 'Failed to create repo'; }
	}

	function logout() {
		localStorage.removeItem('forge_token');
		user = null;
		window.location.href = '/auth';
	}

	onMount(() => {
		fetchUser();
		fetchRepos();
	});
</script>

<div class="flex h-screen overflow-hidden">
	<!-- Sidebar -->
	<aside class="w-64 glass border-r border-surface-700/50 flex flex-col shrink-0 animate-fade-in">
		<div class="flex items-center gap-3 px-5 h-16 border-b border-surface-700/30">
			<div class="w-8 h-8 rounded-lg bg-gradient-to-br from-forge-400 to-accent-500 flex items-center justify-center">
				<FileCode2 size={16} class="text-white" />
			</div>
			<div>
				<h1 class="font-semibold text-surface-100 text-sm tracking-tight">Forge</h1>
				<p class="text-xs text-surface-400">by Veloro</p>
			</div>
		</div>

		<div class="flex-1 overflow-y-auto p-3 space-y-1">
			{#if user}
				<div class="flex items-center gap-2 px-3 py-2 text-xs text-surface-400 mb-3">
					<div class="w-6 h-6 rounded-full bg-gradient-to-br from-forge-400 to-forge-600 flex items-center justify-center text-[10px] font-medium text-white">
						{user.username[0].toUpperCase()}
					</div>
					<span class="truncate">{user.username}</span>
					<span class="ml-auto text-surface-500">{user.repo_count}</span>
				</div>
			{/if}

			<button
				onclick={() => showCreateModal = true}
				class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm text-surface-300 
					   hover:bg-surface-800/50 hover:text-surface-100 transition-all duration-200 group"
			>
				<div class="w-5 h-5 rounded-md bg-forge-500/10 flex items-center justify-center group-hover:bg-forge-500/20 transition-colors">
					<Plus size={14} class="text-forge-400" />
				</div>
				<span>New Repository</span>
			</button>

			<div class="pt-3 pb-1 px-3 text-[11px] font-medium text-surface-500 uppercase tracking-wider">
				Repositories
			</div>

			{#each repos as repo}
				<a
					href="/{repo.owner_username}/{repo.name}"
					class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-surface-300 
						   hover:bg-surface-800/40 hover:text-surface-100 transition-all duration-200 group"
				>
					<GitBranch size={14} class="text-surface-500 group-hover:text-forge-400 transition-colors shrink-0" />
					<span class="truncate">{repo.name}</span>
					{#if repo.is_private}
						<span class="ml-auto text-[10px] px-1.5 py-0.5 rounded bg-surface-800 text-surface-400">Private</span>
					{/if}
				</a>
			{/each}

			{#if loading}
				<div class="space-y-2 px-3 pt-2">
					<div class="h-5 bg-surface-800/50 rounded animate-pulse" />
					<div class="h-5 bg-surface-800/50 rounded animate-pulse w-3/4" />
				</div>
			{/if}
		</div>

		<div class="p-3 border-t border-surface-700/30 space-y-1">
			<a
				href="/dashboard/settings"
				class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm text-surface-400 
					   hover:bg-surface-800/30 hover:text-surface-200 transition-all duration-200"
			>
				<Settings size={14} />
				Settings
			</a>
			<button
				onclick={logout}
				class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-sm text-surface-400 
					   hover:bg-red-500/10 hover:text-red-400 transition-all duration-200"
			>
				<LogOut size={14} />
				Logout
			</button>
		</div>
	</aside>

	<!-- Main content -->
	<main class="flex-1 overflow-y-auto">
		<!-- Header -->
		<header class="h-16 border-b border-surface-800/50 flex items-center justify-between px-6 glass">
			<div class="flex items-center gap-3">
				<h2 class="text-lg font-medium text-surface-100">Dashboard</h2>
				<span class="text-xs text-surface-500 bg-surface-800/50 px-2 py-0.5 rounded-full">
					{repos.length} repos
				</span>
			</div>
			<a
				href="https://github.com/Velooroo/Forge"
				target="_blank"
				class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm text-surface-400 
					   hover:bg-surface-800/40 hover:text-surface-200 transition-all duration-200"
			>
				<Github size={16} />
				<span class="hidden sm:inline">Source</span>
			</a>
		</header>

		<!-- Content -->
		<div class="p-6 space-y-6">
			{#if repos.length === 0 && !loading}
				<div class="flex flex-col items-center justify-center py-20 animate-fade-in">
					<div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-forge-500/20 to-accent-500/20 flex items-center justify-center mb-4">
						<GitBranch size={32} class="text-forge-400" />
					</div>
					<h3 class="text-lg font-medium text-surface-300 mb-2">No repositories yet</h3>
					<p class="text-sm text-surface-500 mb-6">Create your first repository to get started</p>
					<button
						onclick={() => showCreateModal = true}
						class="px-4 py-2 bg-forge-500 hover:bg-forge-400 text-white rounded-lg text-sm font-medium 
							   transition-all duration-200 hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.98]"
					>
						Create Repository
					</button>
				</div>
			{/if}

			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each repos as repo, i}
					<a
						href="/{repo.owner_username}/{repo.name}"
						class="glass rounded-xl p-4 card-hover animate-slide-up"
						style="animation-delay: {i * 0.05}s"
					>
						<div class="flex items-start justify-between mb-3">
							<div class="flex items-center gap-2">
								<GitBranch size={16} class="text-forge-400 shrink-0" />
								<span class="font-medium text-sm text-surface-200 truncate">{repo.name}</span>
							</div>
							{#if repo.is_private}
								<span class="text-[10px] px-1.5 py-0.5 rounded bg-surface-800 text-surface-400 shrink-0">Private</span>
							{/if}
						</div>
						<p class="text-xs text-surface-400 line-clamp-2 mb-3">
							{repo.description || 'No description'}
						</p>
						<div class="flex items-center gap-2 text-xs text-surface-500">
							<ChevronRight size={12} />
							<span class="truncate">{repo.owner_username}/{repo.name}</span>
						</div>
					</a>
				{/each}
			</div>
		</div>
	</main>
</div>

<!-- Create Repo Modal -->
{#if showCreateModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm animate-fade-in"
		onclick={() => showCreateModal = false}
		role="dialog"
	>
		<div
			class="glass rounded-2xl p-6 w-full max-w-md mx-4 animate-scale-in"
			onclick={(e) => e.stopPropagation()}
		>
			<h3 class="text-lg font-medium text-surface-100 mb-1">New Repository</h3>
			<p class="text-sm text-surface-400 mb-5">Create a new repository for your project</p>

			<div class="space-y-4">
				<div>
					<label for="repo-name" class="block text-xs font-medium text-surface-300 mb-1.5">Name</label>
					<input
						id="repo-name"
						type="text"
						placeholder="my-awesome-project"
						bind:value={newRepoName}
						class="input-base text-sm"
					/>
				</div>

				<div>
					<label for="repo-desc" class="block text-xs font-medium text-surface-300 mb-1.5">Description (optional)</label>
					<textarea
						id="repo-desc"
						placeholder="A short description..."
						bind:value={newRepoDesc}
						class="input-base text-sm resize-none h-20"
					></textarea>
				</div>

				<label class="flex items-center gap-3 cursor-pointer group">
					<input
						type="checkbox"
						bind:checked={newRepoPrivate}
						class="w-4 h-4 rounded border-surface-600 bg-surface-800 text-forge-500 
							   focus:ring-forge-500/30 focus:ring-offset-0 cursor-pointer"
					/>
					<div class="flex flex-col">
						<span class="text-sm text-surface-200 group-hover:text-surface-100 transition-colors">Private repository</span>
						<span class="text-xs text-surface-500">Only you and collaborators can access</span>
					</div>
				</label>

				{#if error}
					<div class="text-sm text-red-400 bg-red-500/10 rounded-lg px-3 py-2">
						{error}
					</div>
				{/if}

				<div class="flex gap-3 pt-2">
					<button
						onclick={() => showCreateModal = false}
						class="flex-1 px-4 py-2 rounded-lg text-sm text-surface-300 
							   hover:bg-surface-800/50 transition-all duration-200"
					>
						Cancel
					</button>
					<button
						onclick={createRepo}
						disabled={!newRepoName.trim()}
						class="flex-1 px-4 py-2 bg-forge-500 hover:bg-forge-400 disabled:bg-surface-700 disabled:text-surface-500 
							   text-white rounded-lg text-sm font-medium transition-all duration-200 
							   hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.98]"
					>
						Create
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}