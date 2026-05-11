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

<!-- Sidebar = весь фон экрана, контент — карточка внутри -->
<div class="flex min-h-screen bg-[#0c0d0f] text-white">
	<!-- Nav panel (часть сайдбара) -->
	<aside class="flex w-56 shrink-0 flex-col border-r border-white/[0.06]">
		<div class="flex items-center gap-3 px-5 py-4">
			<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-red-500 to-rose-600">
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

	<!-- Правая часть — тоже фон сайдбара, внутри скруглённая карточка контента -->
	<div class="flex flex-1 flex-col">
		<!-- Поиск сверху -->
		<div class="border-b border-white/[0.06] px-6 py-3">
			<div class="relative max-w-md">
				<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/30" />
				<input
					type="text"
					placeholder="Search..."
					class="h-9 w-full rounded-lg border border-white/[0.06] bg-white/[0.04] pl-10 pr-4 text-sm text-white/60 placeholder-white/20 outline-none transition-all focus:border-red-400/30 focus:bg-white/[0.06]"
				/>
			</div>
		</div>

		<!-- Контент с отступами — скруглённая карточка как экран внутри рамки -->
		<div class="flex-1 px-6 py-5">
			<div class="h-full rounded-2xl bg-[#08090b] p-6">
				{#if mounted}
					<div in:fade={{ duration: 200, easing: cubicOut }}>
						{@render children()}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
