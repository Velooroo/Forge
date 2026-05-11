<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide, scale } from 'svelte/transition';
	import { cubicOut, quintOut } from 'svelte/easing';
	import {
		Folder,
		File,
		GitBranch,
		Download,
		Globe,
		Lock,
		Eye,
		ChevronRight,
		ChevronDown,
		Clock,
		MessageSquare,
		GitCommit,
		User,
		Copy
	} from 'lucide-svelte';
	import { getRepo, updateVisibility, browseTree, type Repo, type TreeEntry } from '../../../../api/repos';

	let { params } = $props();
	let repo = $state<Repo | null>(null);
	let loading = $state(true);
	let error = $state('');
	let coped = $state(false);
	let showVisMenu = $state(false);
	let allRepos = $state<Repo[]>([]);

	interface TreeNode {
		name: string;
		type: 'tree' | 'blob';
		expanded?: boolean;
		children?: TreeNode[];
		loaded?: boolean;
	}

	let treeData = $state<TreeNode[]>([]);

	onMount(async () => {
		try {
			const res = await fetch('http://localhost:8080/api/repos/list');
			allRepos = await res.json();
			repo = allRepos.find((r: Repo) => r.name === params.repo) || null;
			if (!repo) { error = 'Repository not found'; return; }

			const entries = await browseTree(repo.id);
			treeData = entries.map((e: TreeEntry) => ({
				name: e.name,
				type: e.type as 'tree' | 'blob',
				expanded: false,
				children: e.type === 'tree' ? [] : undefined,
				loaded: false
			}));
		} catch (e) {
			error = 'Repository not found';
		} finally {
			loading = false;
		}
	});

	async function toggleFolder(item: TreeNode) {
		if (item.type !== 'tree') return;
		item.expanded = !item.expanded;
		if (item.expanded && !item.loaded) {
			item.loaded = true;
			try {
				const entries = await browseTree(repo!.id, item.name);
				item.children = entries.map((e: TreeEntry) => ({
					name: e.name,
					type: e.type as 'tree' | 'blob',
					expanded: false,
					children: e.type === 'tree' ? [] : undefined,
					loaded: false
				}));
			} catch {}
		}
	}

	async function loadSubTree(item: TreeNode, parentPath: string) {
		const path = parentPath ? `${parentPath}/${item.name}` : item.name;
		if (item.type !== 'tree' || item.loaded) return;
		item.loaded = true;
		try {
			const entries = await browseTree(repo!.id, path);
			item.children = entries.map((e: TreeEntry) => ({
				name: e.name,
				type: e.type as 'tree' | 'blob',
				expanded: false,
				children: e.type === 'tree' ? [] : undefined,
				loaded: false
			}));
		} catch {}
	}

	async function changeVis(v: string) {
		if (!repo) return;
		try {
			await updateVisibility(repo.id, v);
			repo = { ...repo, visibility: v };
		} catch (e) {
			console.error('Failed to update visibility:', e);
		}
		showVisMenu = false;
	}

	async function copyUrl() {
		if (!repo) return;
		try {
			await navigator.clipboard.writeText(repo.clone_url);
			coped = true;
			setTimeout(() => coped = false, 2000);
		} catch {}
	}
</script>

<div class="p-6 lg:p-8">
	{#if loading}
		<div class="flex items-center justify-center py-20">
			<div class="h-8 w-8 animate-spin rounded-full border-2 border-emerald-400 border-t-transparent" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center py-20 text-white/30">
			<File class="mb-4 h-12 w-12" />
			<p class="text-lg font-medium">{error}</p>
		</div>
	{:else if repo}
		<div in:fade={{ duration: 300, easing: cubicOut }}>
			<!-- BREADCRUMB -->
			<div class="mb-6 flex items-center gap-2 text-sm text-white/40">
				<a href="/dashboard" class="transition-colors hover:text-white/60">Repositories</a>
				<ChevronRight class="h-3 w-3" />
				<a href="/dashboard/{params.user}" class="transition-colors hover:text-white/60">{params.user}</a>
				<ChevronRight class="h-3 w-3" />
				<span class="text-white/80">{repo.name}</span>
			</div>

			<!-- REPO HEADER -->
			<div class="mb-6 rounded-2xl border border-white/10 bg-white/[0.04] p-6">
				<div class="flex items-start justify-between">
					<div class="min-w-0 flex-1">
						<div class="flex items-center gap-3">
							<h1 class="truncate text-2xl font-bold">{repo.name}</h1>
							<span
								class="shrink-0 rounded-full border px-2.5 py-0.5 text-[11px] font-medium uppercase tracking-wider {repo.visibility === 'public' ? 'border-emerald-400/20 text-emerald-400/60' : repo.visibility === 'internal' ? 'border-blue-400/20 text-blue-400/60' : 'border-white/10 text-white/40'}"
							>
								{repo.visibility}
							</span>
						</div>
						{#if repo.description}
							<p class="mt-1 text-sm text-white/40">{repo.description}</p>
						{/if}
						<div class="mt-4 flex items-center gap-4 text-xs text-white/30">
							<span class="flex items-center gap-1">
								<GitBranch class="h-3 w-3" /> main
							</span>
							<span class="flex items-center gap-1">
								<Clock class="h-3 w-3" /> just now
							</span>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<!-- Visibility dropdown -->
						<div class="relative">
							<button
								onclick={() => (showVisMenu = !showVisMenu)}
								class="flex items-center gap-2 rounded-xl border border-white/10 px-4 py-2 text-xs font-medium text-white/60 transition-all hover:bg-white/5 hover:text-white"
							>
								{#if repo.visibility === 'public'}
									<Globe class="h-3 w-3" />
								{:else if repo.visibility === 'internal'}
									<Eye class="h-3 w-3" />
								{:else}
									<Lock class="h-3 w-3" />
								{/if}
								Change Visibility
							</button>
							{#if showVisMenu}
								<div class="absolute right-0 top-full z-20 mt-1 w-44 rounded-xl border border-white/10 bg-[#0a0e10] p-1 shadow-2xl backdrop-blur-xl"
									transition:scale={{ duration: 150, start: 0.95 }}>
									<button onclick={() => changeVis('private')}
										class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-xs text-white/60 transition-all hover:bg-white/5 hover:text-white">
										<Lock class="h-3 w-3" /> Private
									</button>
									<button onclick={() => changeVis('internal')}
										class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-xs text-white/60 transition-all hover:bg-white/5 hover:text-white">
										<Eye class="h-3 w-3" /> Internal
									</button>
									<button onclick={() => changeVis('public')}
										class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-xs text-white/60 transition-all hover:bg-white/5 hover:text-white">
										<Globe class="h-3 w-3" /> Public
									</button>
								</div>
							{/if}
						</div>

						<a href={repo.clone_url.replace('http://localhost:8080/git/', 'http://localhost:8080/git/') + '/archive'}
							class="flex items-center gap-2 rounded-xl border border-white/10 px-4 py-2 text-xs font-medium text-white/60 transition-all hover:bg-white/5 hover:text-white">
							<Download class="h-3 w-3" /> Download
						</a>
					</div>
				</div>

				<!-- Quick clone -->
				<div class="mt-4 flex items-center gap-2 rounded-xl bg-white/5 px-4 py-3">
					<code class="flex-1 text-xs text-white/40 font-mono">{repo.clone_url}</code>
					<button onclick={copyUrl}
						class="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs text-white/40 transition-all hover:bg-white/10 hover:text-emerald-400">
						<Copy class="h-3 w-3" />
						{coped ? 'Copied!' : 'Copy'}
					</button>
				</div>
			</div>

			<!-- FILE TREE + README -->
			<div class="grid gap-6 lg:grid-cols-[320px_1fr]">
				<!-- FILE TREE -->
				<div class="rounded-2xl border border-white/10 bg-white/[0.04] p-4"
					in:fade={{ duration: 300, delay: 100, easing: cubicOut }}>
					<div class="mb-3 flex items-center gap-2 text-xs font-medium text-white/40 uppercase tracking-wider">
						<Folder class="h-3 w-3" /> Files
					</div>
					<div class="space-y-0.5">
						{#each treeData as item}
							<button
								onclick={() => toggleFolder(item)}
								class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-white/60 transition-all hover:bg-white/5 hover:text-white"
							>
								{#if item.type === 'tree'}
									{#if item.expanded}
										<ChevronDown class="h-3 w-3 shrink-0" />
									{:else}
										<ChevronRight class="h-3 w-3 shrink-0" />
									{/if}
									<Folder class="h-4 w-4 shrink-0 text-amber-400/60" />
								{:else}
									<span class="w-3 shrink-0" />
									<File class="h-4 w-4 shrink-0 text-blue-400/60" />
								{/if}
								<span class="truncate">{item.name}</span>
							</button>
							{#if item.type === 'tree' && item.expanded && item.children}
								<div class="ml-4 border-l border-white/10 pl-2" in:slide={{ duration: 200 }}>
									{#each item.children as child}
										<button
											class="flex w-full items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-white/50 transition-all hover:bg-white/5 hover:text-white"
										>
											{#if child.type === 'tree'}
												<Folder class="h-4 w-4 shrink-0 text-amber-400/60" />
											{:else}
												<File class="h-4 w-4 shrink-0 text-blue-400/60" />
											{/if}
											<span class="truncate">{child.name}</span>
										</button>
										{#if child.type === 'tree' && child.children}
											{#each child.children as grandchild}
												<button
													class="ml-4 flex w-full items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-white/50 transition-all hover:bg-white/5 hover:text-white"
												>
													<File class="h-4 w-4 shrink-0 text-blue-400/60" />
													<span class="truncate">{grandchild.name}</span>
												</button>
											{/each}
										{/if}
									{/each}
								</div>
							{/if}
						{/each}
					</div>
				</div>

				<!-- README / CONTENT -->
				<div class="rounded-2xl border border-white/10 bg-white/[0.04] p-6"
					in:fade={{ duration: 300, delay: 200, easing: cubicOut }}>
					<div class="flex items-center gap-2 text-xs font-medium text-white/40 uppercase tracking-wider">
						<MessageSquare class="h-3 w-3" /> README.md
					</div>
					<div class="prose prose-invert mt-4 max-w-none text-sm text-white/60">
						<h1 class="text-white/80">{repo.name}</h1>
						{#if repo.description}
							<p>{repo.description}</p>
						{:else}
							<p>No description provided.</p>
						{/if}
						<hr class="border-white/10" />
						<h2>Getting Started</h2>
						<pre class="rounded-xl bg-white/5 p-4 text-xs"><code>git clone {repo.clone_url}
cd {repo.name.split('/').pop() || repo.name}
					</code></pre>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
