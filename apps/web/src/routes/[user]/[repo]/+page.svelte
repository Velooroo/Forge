<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
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
	<!-- Breadcrumb — no border -->
	<div class="flex items-center justify-between px-5 h-10 shrink-0">
		<div class="flex items-center gap-2 text-[11px] text-surface-500 min-w-0">
			<a href="/" class="hover:text-surface-300 transition-colors flex items-center gap-1 shrink-0">
				<ArrowLeft size={10} />
			</a>
			<span class="text-surface-600">/</span>
			<span class="text-surface-400 truncate">{user}/{repoName}</span>
		</div>
		{#if repo}
			<a href={repo.clone_url}
				class="flex items-center gap-1 px-2.5 py-1 rounded-lg text-[10px] font-medium bg-forge-500/10 text-forge-400 
					   hover:bg-forge-500/20 transition-all duration-200 active:scale-[0.97]">
				<Download size={10} /> Clone
			</a>
		{/if}
	</div>

	<div class="flex-1 overflow-y-auto px-4 pb-4">
		{#if loading}
			<div class="space-y-2.5" in:fade={{ duration: 200 }}>
				<div class="h-5 bg-surface-800/20 rounded-lg w-1/4 animate-pulse" />
				<div class="h-3 bg-surface-800/10 rounded w-1/3 animate-pulse" />
				<div class="h-28 bg-surface-800/5 rounded-xl animate-pulse mt-3" />
			</div>
		{:else if error}
			<div class="flex flex-col items-center justify-center h-full" in:fade={{ duration: 200 }}>
				<div class="w-10 h-10 rounded-xl bg-accent-500/10 flex items-center justify-center mb-3">
					<GitBranch size={20} class="text-accent-400" />
				</div>
				<p class="text-xs text-surface-500">{error}</p>
				<a href="/" class="mt-3 text-[11px] text-forge-400 hover:text-forge-300 transition-colors">Back</a>
			</div>
		{:else if repo}
			<div in:fade={{ duration: 250 }}>
				<div class="flex items-start justify-between mb-4">
					<div>
						<div class="flex items-center gap-2">
							<h1 class="text-lg font-bold text-surface-100">{repo.name}</h1>
							<span class="flex items-center gap-1 text-[9px] px-1.5 py-0.5 rounded-full bg-surface-800/50 text-surface-500">
								{#if repo.is_private}<Lock size={8} /> Private{:else}<Globe size={8} /> Public{/if}
							</span>
						</div>
						<p class="text-xs text-surface-500 mt-1">{repo.description || 'No description'}</p>
						<div class="flex items-center gap-3 mt-2 text-[10px] text-surface-600">
							<span class="flex items-center gap-1"><User size={10} /> {repo.owner_username}</span>
							<span class="flex items-center gap-1"><Clock size={10} /> {new Date(repo.created_at).toLocaleDateString()}</span>
						</div>
					</div>
				</div>

				<!-- Tabs -->
				<div class="flex gap-3 mb-3 border-b border-surface-800/20">
					<button onclick={() => activeTab = 'code'}
							class="flex items-center gap-1.5 pb-2 text-[11px] font-medium border-b-2 transition-all duration-200"
							class:border-forge-400={activeTab === 'code'} class:text-forge-400={activeTab === 'code'}
							class:border-transparent={activeTab !== 'code'} class:text-surface-500={activeTab !== 'code'}>
						<Code2 size={13} /> Code
					</button>
					<button onclick={() => activeTab = 'collaborators'}
							class="flex items-center gap-1.5 pb-2 text-[11px] font-medium border-b-2 transition-all duration-200"
							class:border-forge-400={activeTab === 'collaborators'} class:text-forge-400={activeTab === 'collaborators'}
							class:border-transparent={activeTab !== 'collaborators'} class:text-surface-500={activeTab !== 'collaborators'}>
						<Users size={13} /> Collaborators
					</button>
				</div>

				{#key activeTab}
					{#if activeTab === 'code'}
						<div class="bg-surface-800/5 border border-surface-800/20 rounded-xl overflow-hidden" in:fade={{ duration: 150 }}>
							<div class="flex items-center gap-3 px-3.5 py-2 border-b border-surface-800/15 bg-surface-800/5 text-[10px] text-surface-500">
								<span class="flex items-center gap-1"><GitBranch size={10} class="text-forge-400" /> main</span>
								<span class="flex items-center gap-1"><GitCommit size={10} /> No commits yet</span>
							</div>
							<div class="p-6 text-center">
								<div class="w-12 h-12 rounded-xl bg-forge-500/10 flex items-center justify-center mx-auto mb-3">
									<Folder size={24} class="text-forge-400" />
								</div>
								<p class="text-xs text-surface-500 mb-3">Push your first commit</p>
								<div class="bg-surface-800/30 inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-[10px]">
									<span class="text-surface-500">git clone</span>
									<code class="text-forge-400">{repo.clone_url}</code>
								</div>
							</div>
						</div>
					{:else}
						<div class="bg-surface-800/5 border border-surface-800/20 rounded-xl p-6 text-center" in:fade={{ duration: 150 }}>
							<div class="w-10 h-10 rounded-xl bg-forge-500/10 flex items-center justify-center mx-auto mb-2">
								<Users size={20} class="text-forge-400" />
							</div>
							<p class="text-xs text-surface-500">Collaborators — coming soon</p>
						</div>
					{/if}
				{/key}
			</div>
		{/if}
	</div>
</div>