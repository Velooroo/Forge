<script lang="ts">
	import { LogOut, Cpu, LayoutDashboard, FolderGit2, Settings, User, Search } from 'lucide-svelte';
	import { logout } from '../../api/auth';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	let { children } = $props();
	let mounted = $state(false);
	let hoveredIdx = $state(-1);

	let navRefs: (HTMLElement | null)[] = [];
	let indicatorStyle = $state({ top: 0, height: 0, opacity: 0 });

	const navItems = [
		{ href: '/dashboard', icon: LayoutDashboard, label: 'Dashboard' },
		{ href: '/dashboard', icon: FolderGit2, label: 'Repositories' },
		{ href: '/dashboard/settings', icon: Settings, label: 'Settings' },
	];

	onMount(() => { mounted = true; });

	function onNavHover(idx: number) {
		const el = navRefs[idx];
		if (!el) return;
		const parent = el.parentElement;
		if (!parent) return;
		const parentRect = parent.getBoundingClientRect();
		const rect = el.getBoundingClientRect();
		indicatorStyle = {
			top: rect.top - parentRect.top,
			height: rect.height,
			opacity: 1,
		};
		hoveredIdx = idx;
	}

	function onNavLeave() {
		indicatorStyle = { ...indicatorStyle, opacity: 0 };
		hoveredIdx = -1;
	}
</script>

<div class="flex min-h-screen bg-[#08090b] text-white">
	<!-- Sidebar — тёмный, одна скользящая тень-индикатор -->
	<aside class="flex w-60 shrink-0 flex-col bg-[#0c0d0f]">
		<div class="flex items-center gap-3 px-5 py-5">
			<div class="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-red-500 to-rose-600 shadow-lg shadow-red-500/20">
				<Cpu class="h-5 w-5 text-white" />
			</div>
			<div class="text-lg font-bold text-white/90">Forge</div>
		</div>

		<div class="border-t border-white/[0.06] px-5 py-3">
			<div class="flex items-center gap-3">
				<div class="flex h-9 w-9 items-center justify-center rounded-full bg-red-500/20 text-sm font-semibold text-red-400">U</div>
				<div class="min-w-0 flex-1">
					<div class="truncate text-sm font-medium text-white/70">User</div>
					<div class="truncate text-xs text-white/30">@user</div>
				</div>
			</div>
		</div>

		<nav class="relative flex-1 px-3 py-3" onmouseleave={onNavLeave}>
			<!-- Единый скользящий индикатор -->
			<div
				class="pointer-events-none absolute left-3 right-3 rounded-xl bg-gradient-to-r from-red-500/15 to-red-600/10 shadow-lg shadow-red-500/10 transition-all duration-200 ease-out"
				style="top: {indicatorStyle.top}px; height: {indicatorStyle.height}px; opacity: {indicatorStyle.opacity}"
			></div>

			{#each navItems as item, i}
				<a
					href={item.href}
					bind:this={navRefs[i]}
					onmouseenter={() => onNavHover(i)}
					class="relative flex items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-white/50 transition-all duration-150"
					class:text-white={hoveredIdx === i}
				>
					<svelte:component this={item.icon} class="h-4 w-4" />
					<span>{item.label}</span>
				</a>
			{/each}
		</nav>

		<div class="border-t border-white/[0.06] p-3">
			<button onclick={logout} class="flex w-full items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-white/40 transition-all hover:text-red-400">
				<LogOut class="h-4 w-4" />
				<span>Sign out</span>
			</button>
		</div>
	</aside>

	<!-- Правая часть -->
	<div class="flex flex-1 flex-col bg-[#08090b]">
		<!-- Поиск сверху по центру -->
		<div class="flex items-center justify-center border-b border-white/[0.06] px-6 py-3">
			<div class="relative w-full max-w-lg">
				<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-white/30" />
				<input
					type="text"
					placeholder="Search..."
					class="h-9 w-full rounded-lg border border-white/[0.06] bg-white/[0.04] pl-10 pr-4 text-sm text-white/60 outline-none transition-all placeholder:text-white/20 focus:border-red-400/30 focus:bg-white/[0.06]"
				/>
			</div>
		</div>

		<!-- Контент -->
		<div class="flex-1 px-8 py-6">
			<div class="h-full rounded-2xl bg-[#0c0d0f] p-6">
				{#if mounted}
					<div in:fade={{ duration: 200, easing: cubicOut }}>
						{@render children()}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
