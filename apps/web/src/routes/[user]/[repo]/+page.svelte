<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';
	import {
		GitBranch, GitCommit, Folder, ChevronRight, Download, ArrowLeft,
		Lock, Globe, Clock, User, Code2, Users
	} from 'lucide-svelte';

	let { data } = $props();

	let repo: any = $state(null);
	let loading = $state(true);
	let error = $state('');
	let activeTab = $state<'code' | 'collaborators'>('code');

	const user = $derived($page.params.user);
	const repoName = $derived($page.params.repo);

	const API = 'http://localhost:8080/api';

	onMount(async () => {
		try {
			const res = await fetch(`${API}/repos/list`);
			const allRepos = await res.json();
			repo = allRepos.find((r: any) => r.owner_username === user && r.name === repoName) || null;
			if (!repo) error = 'Repository not found';
		} catch { error = 'Failed to load repository'; }
		loading = false;
	});
</script>

<div class="h-full flex flex-col">
	<header class="flex items-center justify-between px-6 h-12 border-b border-surface-800/30 shrink-0">
		<div class="flex items-center gap-2 text-xs text-surface-500 min-w-0">
			<a href="/" class="hover:text-surface-300 transition-colors shrink-0 flex items-center gap-1">
				<ArrowLeft size={12} /> Dashboard
			</a>
			<ChevronRight size={12} class="shrink-0" />
			<span class="text-surface-400 truncate">{user}/{repoName}</span>
		</div>
		{#if repo}
			<a href={repo.clone_url}
			   class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[11px] font-medium bg-forge-500/10 text-forge-400 
					  hover:bg-forge-500/20 transition-all duration-200 active:scale-[0.97]">
				<Download size={12} /> Clone
			</a>
		{/if}
	</header>

	<div class="flex-1 overflow-y-auto p-5">
		{#if loading}
			<div class="space-y-3" in:fade={{ duration: 200 }}>
				<div class="h-6 bg-surface-800/30 rounded-lg w-1/3 animate-pulse" />
				<div class="h-3 bg-surface-800/20 rounded w-1/2 animate-pulse" />
				<div class="h-32 bg-surface-800/10 rounded-xl animate-pulse mt-4" />
			</div>
		{:else if error}
			<div class="flex flex-col items-center justify-center h-full" in:fade={{ duration: 300 }}>
				<div class="w-12 h-12 rounded-xl bg-accent-500/10 flex items-center justify-center mb-3">
					<GitBranch size={24} class="text-accent-400" />
				</div>
				<p class="text-sm text-surface-500">{error}</p>
				<a href="/" class="mt-4 text-xs text-forge-400 hover:text-forge-300 transition-colors">Back to Dashboard</a>
			</div>
		{:else if repo}
			<div in:fade={{ duration: 300 }}>
				<div class="flex items-start justify-between mb-5">
					<div>
						<div class="flex items-center gap-2.5">
							<h1 class="text-xl font-bold text-surface-100">{repo.name}</h1>
							<span class="flex items-center gap-1 text-[10px] px-2 py-0.5 rounded-full bg-surface-800 text-surface-500">
								{#if repo.is_private}<Lock size={9} /> Private{:else}<Globe size={9} /> Public{/if}
							</span>
						</div>
						<p class="text-xs text-surface-500 mt-1.5">{repo.description || 'No description'}</p>
						<div class="flex items-center gap-3 mt-2.5 text-[11px] text-surface-600">
							<span class="flex items-center gap-1"><User size={11} /> {repo.owner_username}</span>
							<span class="flex items-center gap-1"><Clock size={11} /> Created {new Date(repo.created_at).toLocaleDateString()}</span>
						</div>
					</div>
				</div>

				<div class="flex gap-4 mb-4 border-b border-surface-800/30">
					<button onclick={() => activeTab = 'code'}
							class="flex items-center gap-1.5 pb-2.5 text-xs font-medium border-b-2 transition-all duration-200"
							class:border-forge-400={activeTab === 'code'} class:text-forge-400={activeTab === 'code'}
							class:border-transparent={activeTab !== 'code'} class:text-surface-500={activeTab !== 'code'}>
						<Code2 size={14} /> Code
					</button>
					<button onclick={() => activeTab = 'collaborators'}
							class="flex items-center gap-1.5 pb-2.5 text-xs font-medium border-b-2 transition-all duration-200"
							class:border-forge-400={activeTab === 'collaborators'} class:text-forge-400={activeTab === 'collaborators'}
							class:border-transparent={activeTab !== 'collaborators'} class:text-surface-500={activeTab !== 'collaborators'}>
						<Users size={14} /> Collaborators
					</button>
				</div>

				{#key activeTab}
					{#if activeTab === 'code'}
						<div class="bg-surface-800/10 border border-surface-800/30 rounded-xl overflow-hidden" in:fade={{ duration: 200 }}>
							<div class="flex items-center gap-4 px-4 py-2.5 border-b border-surface-800/30 bg-surface-800/5 text-[11px] text-surface-500">
								<span class="flex items-center gap-1"><GitBranch size={11} class="text-forge-400" /> main</span>
								<span class="flex items-center gap-1"><GitCommit size={11} /> No commits yet</span>
							</div>
							<div class="p-8 text-center">
								<div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-forge-500/10 to-accent-500/10 flex items-center justify-center mx-auto mb-3">
									<Folder size={28} class="text-forge-400" />
								</div>
								<h3 class="text-sm text-surface-400 font-medium mb-1">Empty repository</h3>
								<p class="text-xs text-surface-600 mb-4">Push your first commit to see files here</p>
								<div class="bg-surface-800/30 inline-flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs">
									<span class="text-surface-500">git clone</span>
									<code class="text-forge-400">{repo.clone_url}</code>
								</div>
							</div>
						</div>
					{:else}
						<div class="bg-surface-800/10 border border-surface-800/30 rounded-xl p-8 text-center" in:fade={{ duration: 200 }}>
							<div class="w-12 h-12 rounded-xl bg-forge-500/10 flex items-center justify-center mx-auto mb-3">
								<Users size={22} class="text-forge-400" />
							</div>
							<p class="text-sm text-surface-500">Collaboration management coming soon</p>
						</div>
					{/if}
				{/key}
			</div>
		{/if}
	</div>
</div>