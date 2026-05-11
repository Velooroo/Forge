<script lang="ts">
	import { LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, User } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();
	let mounted = $state(false);
	onMount(() => { mounted = true; });
</script>

<div class="flex min-h-screen bg-[#050608] text-white">
	<!-- Sidebar = стеклянная рамка -->
	<aside class="relative flex w-56 shrink-0 flex-col rounded-l-2xl border-r border-white/[0.06] bg-red-950/20 backdrop-blur-xl shadow-[inset_-1px_0_0_rgba(220,38,38,0.15)]">
		<div class="flex items-center gap-3 px-5 py-4">
			<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-red-500 to-rose-600 shadow-lg shadow-red-500/20">
				<Cpu class="h-4 w-4 text-white" />
			</div>
			<div class="text-sm font-semibold text-white/90">Forge</div>
		</div>

		<div class="border-t border-white/[0.06] px-5 py-3">
			<div class="flex items-center gap-3">
				<div class="flex h-7 w-7 items-center justify-center rounded-full bg-red-500/20 text-[11px] font-semibold text-red-400">U</div>
				<div class="min-w-0 flex-1">
					<div class="truncate text-sm text-white/70">User</div>
					<div class="truncate text-[11px] text-white/40">@user</div>
				</div>
			</div>
		</div>

		<nav class="flex-1 space-y-0.5 px-3 py-3">
			<a href="/dashboard" class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/50 transition-all hover:bg-white/[0.06] hover:text-white">
				<LayoutDashboard class="h-4 w-4" /> Dashboard
			</a>
			<a href="/dashboard" class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/50 transition-all hover:bg-white/[0.06] hover:text-white">
				<FolderGit2 class="h-4 w-4" /> Repositories
			</a>
			<a href="/dashboard/settings" class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/50 transition-all hover:bg-white/[0.06] hover:text-white">
				<Settings class="h-4 w-4" /> Settings
			</a>
		</nav>

		<div class="border-t border-white/[0.06] p-3">
			<button onclick={logout} class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm text-white/40 transition-all hover:bg-white/[0.06] hover:text-red-400">
				<LogOut class="h-4 w-4" /> Sign out
			</button>
		</div>
	</aside>

	<!-- Main content — без рамки -->
	<main class="flex-1 overflow-auto">
		{#if mounted}
			<div in:fade={{ duration: 200, easing: cubicOut }}>
				{@render children()}
			</div>
		{/if}
	</main>
</div>
