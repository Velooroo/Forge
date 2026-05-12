<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, scale } from 'svelte/transition';
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
			const res = await fetch(`${API}/users/me`, { headers: { Authorization: `Bearer ${token}` } });
			if (res.ok) user = await res.json();
		} catch {}
	}

	async function fetchRepos() {
		loading = true;
		try {
			const token = getToken();
			const res = await fetch(`${API}/repos/mine`, { headers: token ? { Authorization: `Bearer ${token}` } : {} });
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
			} else error = await res.text();
		} catch { error = 'Failed to create repo'; }
	}

	onMount(() => { fetchUser(); fetchRepos(); });
</script>

<div class="h-full flex flex-col">
	<!-- Minimal header — no border -->
	<div class="flex items-center justify-between px-5 h-12 shrink-0">
		<div class="flex items-center gap-2.5">
			<h2 class="text-sm font-medium text-surface-200">Projects</h2>
			<span class="text-[10px] text-surface-500 bg-surface-800/30 px-2 py-0.5 rounded-full">{repos.length}</span>
		</div>
		<div class="flex items-center gap-1.5">
			<button onclick={() => showCreateModal = true}
				class="flex items-center gap-1 px-2.5 py-1.5 bg-forge-500 hover:bg-forge-400 text-white rounded-lg text-[11px] font-medium 
					   transition-all duration-200 hover:shadow-md hover:shadow-forge-500/20 active:scale-[0.97]">
				<Plus size={13} /> New
			</button>
			<a href="https://github.com/Velooroo/Forge" target="_blank"
				class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-[11px] text-surface-500 
					   hover:bg-surface-800/30 hover:text-surface-300 transition-all duration-200">
				<Github size={13} />
			</a>
		</div>
	</div>

	<!-- Cards grid -->
	<div class="flex-1 overflow-y-auto px-4 pb-4">
		{#if repos.length === 0 && !loading}
			<div class="flex flex-col items-center justify-center h-full" in:fade={{ duration: 300 }}>
				<div class="w-12 h-12 rounded-xl bg-forge-500/10 flex items-center justify-center mb-3">
					<FolderOpen size={24} class="text-forge-400" />
				</div>
				<p class="text-sm text-surface-500 mb-4">No projects yet</p>
				<button onclick={() => showCreateModal = true}
					class="px-3.5 py-1.5 bg-forge-500 hover:bg-forge-400 text-white rounded-lg text-xs font-medium 
						   transition-all duration-200 hover:shadow-md hover:shadow-forge-500/20 active:scale-[0.97]">
					Create Project
				</button>
			</div>
		{/if}

		{#if loading}
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2.5">
				{#each Array(6) as _, i}
					<div class="bg-surface-800/10 rounded-xl p-3.5 animate-pulse" in:fade={{ duration: 200, delay: i * 40 }}>
						<div class="h-3.5 bg-surface-700/30 rounded w-1/2 mb-2.5" />
						<div class="h-2.5 bg-surface-700/20 rounded w-3/4 mb-2" />
						<div class="h-2.5 bg-surface-700/20 rounded w-1/4" />
					</div>
				{/each}
			</div>
		{:else}
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2.5">
				{#each repos as repo, i}
					<a href="/{repo.owner_username}/{repo.name}"
						class="block bg-surface-800/10 hover:bg-surface-800/25 rounded-xl p-3.5 transition-all duration-200 group"
						in:fade={{ duration: 250, delay: i * 30 }}>
						<div class="flex items-start justify-between mb-2">
							<div class="flex items-center gap-2 min-w-0">
								<GitBranch size={13} class="text-forge-400 shrink-0" />
								<span class="font-medium text-sm text-surface-200 truncate group-hover:text-forge-400 transition-colors">{repo.name}</span>
							</div>
							{#if repo.is_private}
								<span class="text-[9px] px-1.5 py-0.5 rounded bg-surface-800 text-surface-500 shrink-0 ml-2">Private</span>
							{/if}
						</div>
						<p class="text-[11px] text-surface-500 line-clamp-2 mb-2">{repo.description || 'No description'}</p>
						<div class="flex items-center gap-1 text-[10px] text-surface-600">
							<ChevronRight size={9} />
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
		 onkeydown={(e) => e.key === 'Escape' && (showCreateModal = false)} in:fade={{ duration: 120 }}>
		<div class="absolute inset-0" onclick={() => showCreateModal = false} />
		<div class="relative bg-surface-900 backdrop-blur-2xl border border-surface-700/30 rounded-2xl p-5 w-full max-w-sm mx-4 shadow-2xl"
			 in:scale={{ duration: 120, start: 0.95 }}>
			<h3 class="text-sm font-medium text-surface-100 mb-0.5">New Project</h3>
			<p class="text-xs text-surface-500 mb-4">Create a new repository</p>
			<div class="space-y-3.5">
				<div>
					<label for="repo-name" class="block text-xs text-surface-400 mb-1">Name</label>
					<input id="repo-name" type="text" placeholder="my-project" bind:value={newRepoName} class="input-base text-sm" />
				</div>
				<div>
					<label for="repo-desc" class="block text-xs text-surface-400 mb-1">Description</label>
					<textarea id="repo-desc" placeholder="What's this about?" bind:value={newRepoDesc} class="input-base text-sm resize-none h-16"></textarea>
				</div>
				<label class="flex items-center gap-2.5 cursor-pointer group">
					<input type="checkbox" bind:checked={newRepoPrivate}
						   class="w-3.5 h-3.5 rounded border-surface-600 bg-surface-800 text-forge-500 focus:ring-forge-500/30 cursor-pointer" />
					<div class="flex flex-col">
						<span class="text-xs text-surface-300 group-hover:text-surface-100 transition-colors">Private</span>
						<span class="text-[10px] text-surface-500">Only you and collaborators</span>
					</div>
				</label>
				{#if error}
					<div class="text-xs text-red-400 bg-red-500/10 rounded-lg px-3 py-2">{error}</div>
				{/if}
				<div class="flex gap-2.5 pt-1">
					<button onclick={() => showCreateModal = false}
							class="flex-1 px-3 py-1.5 rounded-lg text-xs text-surface-400 hover:bg-surface-800/40 transition-all">Cancel</button>
					<button onclick={createRepo} disabled={!newRepoName.trim()}
							class="flex-1 px-3 py-1.5 bg-forge-500 hover:bg-forge-400 disabled:bg-surface-700 disabled:text-surface-500 
								   text-white rounded-lg text-xs font-medium transition-all active:scale-[0.97]">Create</button>
				</div>
			</div>
		</div>
	</div>
{/if}