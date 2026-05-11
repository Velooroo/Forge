<script lang="ts">
	import { LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, User, Key, ChevronRight } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();

	let mounted = $state(false);

	onMount(() => { mounted = true; });

	const navItems: { href: string; icon: typeof LayoutDashboard; label: string }[] = [
		{ href: '/dashboard', icon: LayoutDashboard, label: 'Dashboard' },
		{ href: '/dashboard', icon: FolderGit2, label: 'Repositories' },
		{ href: '/dashboard/settings', icon: Settings, label: 'Settings' },
	];
</script>

<div class="flex min-h-screen bg-[#08090b] text-white">
	<!-- SIDEBAR (GitLab-style) -->
	<aside
		class="relative flex w-60 shrink-0 flex-col overflow-hidden rounded-r-2xl border-r border-white/[0.06] bg-[#0c0d0f]"
	>
		<!-- Logo area -->
		<div class="flex items-center gap-3 border-b border-white/[0.06] px-5 py-4">
			<div class="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-red-500 to-rose-600 shadow-lg shadow-red-500/20">
				<Cpu class="h-[18px] w-[18px] text-white" />
			</div>
			<div>
				<div class="text-sm font-semibold text-white/90">Forge</div>
				<div class="text-[11px] text-white/30">Git control plane</div>
			</div>
		</div>

		<!-- User context -->
		<div class="border-b border-white/[0.06] px-5 py-3">
			<div class="flex items-center gap-3">
				<div class="flex h-8 w-8 items-center justify-center rounded-full bg-red-500/20 text-xs font-semibold text-red-400">
					U
				</div>
				<div class="min-w-0 flex-1">
					<div class="truncate text-sm text-white/80">User</div>
					<div class="truncate text-[11px] text-white/30">@user</div>
				</div>
			</div>
		</div>

		<!-- Navigation -->
		<nav class="flex-1 space-y-0.5 px-3 py-4">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/50 transition-all hover:bg-white/[0.06] hover:text-white"
				>
					<svelte:component this={item.icon} class="h-4 w-4" />
					<span>{item.label}</span>
				</a>
			{/each}
		</nav>

		<!-- Bottom actions -->
		<div class="border-t border-white/[0.06] p-3">
			<button
				onclick={logout}
				class="flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm text-white/40 transition-all hover:bg-white/[0.06] hover:text-red-400"
			>
				<LogOut class="h-4 w-4" />
				<span>Sign out</span>
			</button>
		</div>
	</aside>

	<!-- MAIN -->
	<main class="flex-1 overflow-auto">
		{#if mounted}
			<div in:fade={{ duration: 200, easing: cubicOut }}>
				{@render children()}
			</div>
		{/if}
	</main>
</div>
