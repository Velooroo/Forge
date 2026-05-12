<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { GitBranch, Plus, ChevronRight, Github, FolderOpen } from 'lucide-svelte';

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
				headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
				body: JSON.stringify({ name: newRepoName.trim(), description: newRepoDesc.trim() || null, is_private: newRepoPrivate })
			});
			if (res.ok) {
				showCreateModal = false;
				newRepoName = ''; newRepoDesc = ''; newRepoPrivate = false;
				await fetchRepos();
			} else {
				error = await res.text();
			}
		} catch { error = 'Failed to create repo'; }
	}

	onMount(() => { fetchUser(); fetchRepos(); });
</script>

<div class="h-full flex flex-col">
	<!-- Header bar -->
	<header class="flex items-center justify-between px-6 h-14 border-b border-surface-800/30 shrink-0">
		<div class="flex items-center gap-3">
			<h2 class="text-base font-medium text-surface-100">Dashboard</h2>
			<span class="text-[11px] text-surface-500 bg-surface-800/50 px-2 py-0.5 rounded-full">{repos.length} repos</span>
		</div>
		<div class="flex items-center gap-2">
			<button
				onclick={() => showCreateModal = true}
				class="flex items-center gap-1.5 px-3 py-1.5 bg-forge-500 hover:bg-forge-400 text-white rounded-lg text-xs font-medium 
					   transition-all duration-200 hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.97]"
			>
				<Plus size={14} /> New
			</button>
			<a href="https://github.com/Velooroo/Forge" target="_blank"
			   class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs text-surface-500 
					  hover:bg-surface-800/30 hover:text-surface-300 transition-all duration-200">
				<Github size={14} /> Source
			</a>
		</div>
	</header>

	<!-- Content -->
	<div class="flex-1 overflow-y-auto p-5">
		{#if repos.length === 0 && !loading}
			<div class="flex flex-col items-center justify-center h-full animate-fade-in">
				<div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-forge-500/15 to-accent-500/15 flex items-center justify-center mb-4">
					<FolderOpen size={28} class="text-forge-400" />
				</div>
				<h3 class="text-base font-medium text-surface-400 mb-1">No repositories yet</h3>
				<p class="text-xs text-surface-600 mb-5">Create your first repository to get started</p>
				<button onclick={() => showCreateModal = true}
					class="px-4 py-2 bg-forge-500 hover:bg-forge-400 text-white rounded-lg text-xs font-medium 
						   transition-all duration-200 hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.97]">
					Create Repository
				</button>
			</div>
		{/if}

		{#if loading}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
				{#each Array(6) as _, i}
					<div class="bg-surface-800/20 rounded-xl p-4 animate-pulse" in:fade={{ duration: 200, delay: i * 50 }}>
						<div class="h-4 bg-surface-700/50 rounded w-1/2 mb-3" />
						<div class="h-3 bg-surface-700/30 rounded w-3/4 mb-2" />
						<div class="h-3 bg-surface-700/30 rounded w-1/3" />
					</div>
				{/each}
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
				{#each repos as repo, i}
					<a href="/{repo.owner_username}/{repo.name}"
					   class="block bg-surface-800/20 hover:bg-surface-800/40 border border-surface-800/20 hover:border-surface-700/30 
							  rounded-xl p-4 transition-all duration-200 group"
					   in:fade={{ duration: 300, delay: i * 40 }}
					>
						<div class="flex items-start justify-between mb-2">
							<div class="flex items-center gap-2 min-w-0">
								<GitBranch size={14} class="text-forge-400 shrink-0" />
								<span class="font-medium text-sm text-surface-200 truncate group-hover:text-forge-400 transition-colors">{repo.name}</span>
							</div>
							{#if repo.is_private}
								<span class="text-[10px] px-1.5 py-0.5 rounded bg-surface-800 text-surface-500 shrink-0 ml-2">Private</span>
							{/if}
						</div>
						<p class="text-xs text-surface-500 line-clamp-2 mb-2">{repo.description || 'No description'}</p>
						<div class="flex items-center gap-1 text-[11px] text-surface-600">
							<ChevronRight size={10} />
							<span class="truncate">{repo.owner_username}/{repo.name}</span>
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- Create Repo Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
		 onkeydown={(e) => e.key === 'Escape' && (showCreateModal = false)}
		 in:fade={{ duration: 150 }}>
		<!-- backdrop -->
		<div class="absolute inset-0" onclick={() => showCreateModal = false} />
<!-- modal -->
		<div class="relative bg-surface-900/90 backdrop-blur-2xl border border-surface-700/40 rounded-2xl p-6 w-full max-w-md mx-4 shadow-2xl"
			 in:scale={{ duration: 150, start: 0.95 }}>
			<h3 class="text-base font-medium text-surface-100 mb-1">New Repository</h3>
			<p class="text-xs text-surface-500 mb-5">Create a new repository for your project</p>

			<div class="space-y-4">
				<div>
					<label for="repo-name" class="block text-xs font-medium text-surface-300 mb-1.5">Name</label>
					<input id="repo-name" type="text" placeholder="my-awesome-project" bind:value={newRepoName} class="input-base text-sm" />
				</div>
				<div>
					<label for="repo-desc" class="block text-xs font-medium text-surface-300 mb-1.5">Description</label>
					<textarea id="repo-desc" placeholder="A short description..." bind:value={newRepoDesc} class="input-base text-sm resize-none h-20"></textarea>
				</div>
				<label class="flex items-center gap-3 cursor-pointer group">
					<input type="checkbox" bind:checked={newRepoPrivate}
						   class="w-4 h-4 rounded border-surface-600 bg-surface-800 text-forge-500 focus:ring-forge-500/30 cursor-pointer" />
					<div class="flex flex-col">
						<span class="text-sm text-surface-200 group-hover:text-surface-100 transition-colors">Private repository</span>
						<span class="text-xs text-surface-500">Only you and collaborators can access</span>
					</div>
				</label>

				{#if error}
					<div class="text-sm text-red-400 bg-red-500/10 rounded-lg px-3 py-2">{error}</div>
				{/if}

				<div class="flex gap-3 pt-2">
					<button onclick={() => showCreateModal = false}
							class="flex-1 px-4 py-2 rounded-lg text-sm text-surface-400 hover:bg-surface-800/40 transition-all">Cancel</button>
					<button onclick={createRepo} disabled={!newRepoName.trim()}
							class="flex-1 px-4 py-2 bg-forge-500 hover:bg-forge-400 disabled:bg-surface-700 disabled:text-surface-500 
								   text-white rounded-lg text-sm font-medium transition-all active:scale-[0.97]">Create</button>
				</div>
			</div>
		</div>
	</div>
{/if}