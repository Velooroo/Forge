<script lang="ts">
	import { LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, User, Search } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();
	let mounted = $state(false);
	onMount(() => { mounted = true; });
</script>

<div class="flex min-h-screen bg-white text-gray-900">
	<!-- Sidebar — яркий красный glass -->
	<aside class="flex w-60 shrink-0 flex-col bg-gradient-to-b from-red-500/90 to-red-600/90 backdrop-blur-xl shadow-xl">
		<div class="flex items-center gap-3 px-5 py-5">
			<div class="flex h-9 w-9 items-center justify-center rounded-xl bg-white/20">
				<Cpu class="h-5 w-5 text-white" />
			</div>
			<div class="text-lg font-bold text-white">Forge</div>
		</div>

		<div class="px-5 py-3">
			<div class="flex items-center gap-3">
				<div class="flex h-9 w-9 items-center justify-center rounded-full bg-white/20 text-sm font-semibold text-white">U</div>
				<div class="min-w-0 flex-1">
					<div class="truncate text-sm font-medium text-white/90">User</div>
					<div class="truncate text-xs text-white/60">@user</div>
				</div>
			</div>
		</div>

		<nav class="flex-1 space-y-1 px-3 py-4">
			<a href="/dashboard" class="group flex items-center gap-3 rounded-xl bg-white/10 px-4 py-3 text-sm font-medium text-white shadow-lg shadow-black/5 transition-all hover:bg-white/20 hover:shadow-xl hover:shadow-black/10">
				<LayoutDashboard class="h-4 w-4" />
				<span>Dashboard</span>
			</a>
			<a href="/dashboard" class="group flex items-center gap-3 rounded-xl bg-white/5 px-4 py-3 text-sm font-medium text-white/80 transition-all hover:bg-white/20 hover:shadow-xl hover:shadow-black/10">
				<FolderGit2 class="h-4 w-4" />
				<span>Repositories</span>
			</a>
			<a href="/dashboard/settings" class="group flex items-center gap-3 rounded-xl bg-white/5 px-4 py-3 text-sm font-medium text-white/80 transition-all hover:bg-white/20 hover:shadow-xl hover:shadow-black/10">
				<Settings class="h-4 w-4" />
				<span>Settings</span>
			</a>
		</nav>

		<div class="px-3 py-3">
			<button onclick={logout} class="flex w-full items-center gap-3 rounded-xl bg-white/5 px-4 py-3 text-sm font-medium text-white/70 transition-all hover:bg-white/20 hover:text-white">
				<LogOut class="h-4 w-4" />
				<span>Sign out</span>
			</button>
		</div>
	</aside>

	<!-- Content area — светлая -->
	<div class="flex flex-1 flex-col bg-gray-50">
		<!-- Top bar с поиском по центру -->
		<div class="flex items-center justify-center border-b border-gray-200/60 bg-white px-6 py-3">
			<div class="relative w-full max-w-lg">
				<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
				<input
					type="text"
					placeholder="Search..."
					class="h-9 w-full rounded-lg border border-gray-200 bg-gray-50 pl-10 pr-4 text-sm text-gray-600 outline-none transition-all placeholder:text-gray-400 focus:border-red-300 focus:bg-white focus:ring-2 focus:ring-red-100"
				/>
			</div>
		</div>

		<!-- Content card — скруглённая, с тенью -->
		<div class="flex-1 px-8 py-6">
			<div class="h-full rounded-2xl border border-gray-200/60 bg-white p-6 shadow-sm">
				{#if mounted}
					<div in:fade={{ duration: 200, easing: cubicOut }}>
						{@render children()}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
