<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import {
		GitBranch, GitCommit, GitPullRequest, Star, GitFork,
		Folder, File, FileText, ChevronRight, Download,
		Lock, Globe, Clock, User
	} from 'lucide-svelte';

	let { data } = $props();

	let repo: any = $state(null);
	let loading = $state(true);
	let error = $state('');
	let activeTab = $state<'code' | 'collaborators' | 'settings'>('code');

	const user = $derived($page.params.user);
	const repoName = $derived($page.params.repo);

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') {
			return localStorage.getItem('forge_token');
		}
		return null;
	}

	onMount(async () => {
		try {
			const res = await fetch(`${API}/repos/list`);
			const allRepos = await res.json();
			repo = allRepos.find((r: any) => r.owner_username === user && r.name === repoName) || null;
			if (!repo) error = 'Repository not found';
		} catch {
			error = 'Failed to load repository';
		}
		loading = false;
	});
</script>

<div class="min-h-screen bg-surface-950">
	<!-- Header -->
	<header class="border-b border-surface-800/50 glass">
		<div class="max-w-6xl mx-auto px-6">
			<div class="flex items-center gap-2 h-12 text-sm text-surface-400">
				<a href="/" class="hover:text-surface-200 transition-colors">Dashboard</a>
				<ChevronRight size={14} />
				<a href="/{user}" class="hover:text-surface-200 transition-colors">{user}</a>
				<ChevronRight size={14} />
				<span class="text-surface-100">{repoName}</span>
			</div>

			{#if repo}
				<div class="flex items-start justify-between pb-4">
					<div>
						<div class="flex items-center gap-3">
							<h1 class="text-2xl font-bold text-surface-100">{repo.name}</h1>
							<span class="flex items-center gap-1 text-xs px-2 py-0.5 rounded-full bg-surface-800 text-surface-400">
								{#if repo.is_private}
									<Lock size={10} /> Private
								{:else}
									<Globe size={10} /> Public
								{/if}
							</span>
						</div>
						<p class="text-sm text-surface-400 mt-1">{repo.description || 'No description'}</p>
						<div class="flex items-center gap-4 mt-3 text-xs text-surface-500">
							<span class="flex items-center gap-1">
								<User size={12} /> {repo.owner_username}
							</span>
							<span class="flex items-center gap-1">
								<Clock size={12} /> {new Date(repo.created_at).toLocaleDateString()}
							</span>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<a
							href={repo.clone_url}
							class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium bg-forge-500 
								   hover:bg-forge-400 text-white transition-all duration-200 hover:shadow-lg hover:shadow-forge-500/25"
						>
							<Download size={14} /> Clone
						</a>
					</div>
				</div>

				<!-- Tabs -->
				<div class="flex gap-1 -mb-px">
					<button
						class="px-4 py-2.5 text-sm font-medium border-b-2 transition-all duration-200"
						class:border-forge-400={activeTab === 'code'}
						class:text-forge-400={activeTab === 'code'}
						class:border-transparent={activeTab !== 'code'}
						class:text-surface-400={activeTab !== 'code'}
						onclick={() => activeTab = 'code'}
					>
						Code
					</button>
					<button
						class="px-4 py-2.5 text-sm font-medium border-b-2 transition-all duration-200"
						class:border-forge-400={activeTab === 'collaborators'}
						class:text-forge-400={activeTab === 'collaborators'}
						class:border-transparent={activeTab !== 'collaborators'}
						class:text-surface-400={activeTab !== 'collaborators'}
						onclick={() => activeTab = 'collaborators'}
					>
						Collaborators
					</button>
				</div>
			{/if}
		</div>
	</header>

	<!-- Content -->
	<div class="max-w-6xl mx-auto px-6 py-6">
		{#if loading}
			<div class="glass rounded-xl p-6 animate-pulse space-y-3">
				<div class="h-4 bg-surface-800 rounded w-1/3" />
				<div class="h-4 bg-surface-800 rounded w-1/2" />
				<div class="h-4 bg-surface-800 rounded w-2/3" />
			</div>
		{:else if error}
			<div class="glass rounded-xl p-12 text-center animate-fade-in">
				<div class="w-12 h-12 rounded-xl bg-red-500/10 flex items-center justify-center mx-auto mb-3">
					<GitBranch size={24} class="text-red-400" />
				</div>
				<p class="text-surface-400 text-sm">{error}</p>
			</div>
		{:else if repo}
			{#if activeTab === 'code'}
				<div class="glass rounded-xl overflow-hidden animate-fade-in">
					<!-- Git info bar -->
					<div class="flex items-center gap-4 px-4 py-2.5 border-b border-surface-700/30 bg-surface-900/30 text-xs text-surface-400">
						<span class="flex items-center gap-1">
							<GitBranch size={12} class="text-forge-400" /> main
						</span>
						<span class="flex items-center gap-1">
							<GitCommit size={12} /> No commits yet
						</span>
					</div>

					<!-- File tree placeholder -->
					<div class="p-8 text-center">
						<div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-forge-500/10 to-accent-500/10 flex items-center justify-center mx-auto mb-4">
							<Folder size={32} class="text-forge-400" />
						</div>
						<h3 class="text-surface-300 font-medium mb-2">Empty repository</h3>
						<p class="text-sm text-surface-500 mb-6">
							Push your first commit to see files here
						</p>
						<div class="glass inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs">
							<span class="text-surface-400">git clone</span>
							<code class="text-forge-400">{repo.clone_url}</code>
						</div>
					</div>
				</div>
			{:else if activeTab === 'collaborators'}
				<div class="glass rounded-xl p-6 animate-fade-in">
					<div class="text-center py-8">
						<div class="w-12 h-12 rounded-xl bg-forge-500/10 flex items-center justify-center mx-auto mb-3">
							<User size={24} class="text-forge-400" />
						</div>
						<p class="text-surface-400 text-sm">Collaboration management coming soon</p>
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>