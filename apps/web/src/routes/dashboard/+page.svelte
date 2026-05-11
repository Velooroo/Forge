<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide, scale } from 'svelte/transition';
	import { cubicOut, quintOut } from 'svelte/easing';
	import { Plus, FolderGit2, Globe, Lock, Eye, ArrowRight, GitFork, Search, Cpu } from 'lucide-svelte';
	import { listRepos, createRepo, type Repo } from '../../api/repos';
	import { isAuthenticated } from '../../api/auth';

	let repos = $state<Repo[]>([]);
	let loading = $state(true);
	let showCreate = $state(false);
	let newName = $state('');
	let newDesc = $state('');
	let newVis = $state('private');
	let searchQuery = $state('');

	onMount(async () => {
		if (!isAuthenticated()) {
			window.location.href = '/';
			return;
		}
		try {
			repos = await listRepos();
		} catch (e) {
			console.error('Failed to load repos:', e);
		} finally {
			loading = false;
		}
	});

	async function handleCreate() {
		try {
			const repo = await createRepo({ name: newName, description: newDesc, visibility: newVis });
			repos = [repo, ...repos];
			showCreate = false;
			newName = '';
			newDesc = '';
			newVis = 'private';
		} catch (e) {
			console.error('Failed to create repo:', e);
		}
	}

	function visibilityIcon(v: string) {
		if (v === 'public') return Globe;
		if (v === 'internal') return Eye;
		return Lock;
	}

	let filtered = $derived(searchQuery
		? repos.filter((r) => r.name.toLowerCase().includes(searchQuery.toLowerCase()))
		: repos);
</script>

<div class="p-6 lg:p-8">
	<!-- HEADER -->
	<div class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Repositories</h1>
			<p class="mt-1 text-sm text-white/40">Manage your Git repositories</p>
		</div>
		<button
			onclick={() => (showCreate = true)}
			class="flex items-center gap-2 rounded-2xl bg-gradient-to-r from-emerald-500 to-cyan-500 px-5 py-3 text-sm font-semibold text-white shadow-lg shadow-emerald-500/15 transition-all hover:scale-[1.02] hover:shadow-emerald-500/25"
		>
			<Plus class="h-4 w-4" />
			New Repository
		</button>
	</div>

	<!-- SEARCH -->
	<div class="relative mb-6 max-w-md">
		<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/30" />
		<input
			type="text"
			placeholder="Search repositories..."
			bind:value={searchQuery}
			class="h-11 w-full rounded-xl border border-white/10 bg-white/5 pl-10 pr-4 text-sm text-white placeholder-white/25 transition-all focus:border-emerald-400/40 focus:bg-white/10 focus:outline-none focus:ring-emerald-500/20"
		/>
	</div>

	<!-- CREATE MODAL -->
	{#if showCreate}
		<div
			class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
			onclick={() => (showCreate = false)}
			role="presentation"
		>
			<div
				transition:scale={{ duration: 200, easing: quintOut, start: 0.95 }}
				class="w-full max-w-lg rounded-3xl border border-white/10 bg-[#0a0e10] p-8 shadow-2xl"
				onclick={(e) => e.stopPropagation()}
				role="presentation"
			>
				<h2 class="text-xl font-bold">Create Repository</h2>
				<p class="mt-1 text-sm text-white/40">A new Git repo will be created on the server</p>

				<div class="mt-6 space-y-4">
					<div>
						<label class="mb-1.5 block text-xs font-medium text-white/60">Name</label>
						<input
							type="text"
							placeholder="my-awesome-project"
							bind:value={newName}
							class="h-12 w-full rounded-xl border border-white/10 bg-white/5 px-4 text-sm text-white placeholder-white/25 transition-all focus:border-emerald-400/40 focus:bg-white/10 focus:outline-none focus:ring-emerald-500/20"
						/>
					</div>
					<div>
						<label class="mb-1.5 block text-xs font-medium text-white/60">Description (optional)</label>
						<input
							type="text"
							placeholder="What does this project do?"
							bind:value={newDesc}
							class="h-12 w-full rounded-xl border border-white/10 bg-white/5 px-4 text-sm text-white placeholder-white/25 transition-all focus:border-emerald-400/40 focus:bg-white/10 focus:outline-none focus:ring-emerald-500/20"
						/>
					</div>
					<div>
						<label class="mb-1.5 block text-xs font-medium text-white/60">Visibility</label>
						<div class="grid grid-cols-3 gap-2">
							<button
								onclick={() => (newVis = 'private')}
								class="flex items-center justify-center gap-2 rounded-xl border px-3 py-2.5 text-xs font-medium transition-all {newVis === 'private' ? 'border-emerald-400/40 bg-emerald-500/10' : 'border-white/10 bg-white/5'}"
							>
								<Lock class="h-3 w-3" /> Private
							</button>
							<button
								onclick={() => (newVis = 'internal')}
								class="flex items-center justify-center gap-2 rounded-xl border px-3 py-2.5 text-xs font-medium transition-all {newVis === 'internal' ? 'border-emerald-400/40 bg-emerald-500/10' : 'border-white/10 bg-white/5'}"
							>
								<Eye class="h-3 w-3" /> Internal
							</button>
							<button
								onclick={() => (newVis = 'public')}
								class="flex items-center justify-center gap-2 rounded-xl border px-3 py-2.5 text-xs font-medium transition-all {newVis === 'public' ? 'border-emerald-400/40 bg-emerald-500/10' : 'border-white/10 bg-white/5'}"
							>
								<Globe class="h-3 w-3" /> Public
							</button>
						</div>
					</div>
				</div>

				<div class="mt-8 flex items-center justify-end gap-3">
					<button
						onclick={() => (showCreate = false)}
						class="rounded-xl border border-white/10 px-5 py-2.5 text-sm font-medium text-white/60 transition-all hover:bg-white/5 hover:text-white"
					>
						Cancel
					</button>
					<button
						onclick={handleCreate}
						disabled={!newName}
						class="rounded-xl bg-gradient-to-r from-emerald-500 to-cyan-500 px-5 py-2.5 text-sm font-semibold text-white shadow-lg shadow-emerald-500/15 transition-all hover:scale-[1.02] disabled:cursor-not-allowed disabled:opacity-50"
					>
						Create
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- REPO LIST -->
	{#if loading}
		<div class="flex items-center justify-center py-20">
			<div class="h-8 w-8 animate-spin rounded-full border-2 border-emerald-400 border-t-transparent" />
		</div>
	{:else if filtered.length === 0}
		<div class="flex flex-col items-center justify-center py-20 text-white/30">
			<FolderGit2 class="mb-4 h-12 w-12" />
			<p class="text-lg font-medium">No repositories yet</p>
			<p class="mt-1 text-sm">Create your first repository to get started</p>
		</div>
	{:else}
		<div class="grid gap-3">
			{#each filtered as repo, i (repo.id)}
				<div
					class="group flex items-center gap-4 rounded-2xl border border-white/10 bg-white/[0.04] p-4 transition-all hover:border-white/20 hover:bg-white/[0.07]"
					in:fade={{ duration: 300, delay: i * 50, easing: cubicOut }}
				>
					<div
						class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br from-emerald-400/20 to-cyan-400/20"
					>
						<svelte:component this={visibilityIcon(repo.visibility)} class="h-4 w-4 text-emerald-400/60" />
					</div>

					<div class="min-w-0 flex-1">
						<a
							href="/dashboard/kazilsky/{repo.name}"
							class="font-medium text-white/80 transition-colors hover:text-emerald-400"
						>
							{repo.name}
						</a>
						{#if repo.description}
							<p class="mt-0.5 truncate text-sm text-white/40">{repo.description}</p>
						{/if}
					</div>

					<div class="hidden items-center gap-4 sm:flex">
						<span
							class="rounded-full border px-2.5 py-0.5 text-[11px] font-medium uppercase tracking-wider {repo.visibility === 'private' ? 'border-white/10 text-white/40' : repo.visibility === 'public' ? 'border-emerald-400/20 text-emerald-400/60' : 'border-blue-400/20 text-blue-400/60'}"
						>
							{repo.visibility}
						</span>
						<code class="rounded-lg bg-white/5 px-3 py-1.5 text-xs text-white/30 font-mono">
							{repo.clone_url}
						</code>
					</div>

					<a
						href="/dashboard/kazilsky/{repo.name}"
						class="flex h-8 w-8 items-center justify-center rounded-lg text-white/20 transition-all hover:bg-white/10 hover:text-emerald-400"
					>
						<ArrowRight class="h-4 w-4" />
					</a>
				</div>
			{/each}
		</div>
	{/if}
</div>
