<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import {
		GitBranch, Terminal as TerminalIcon, FileCode2,
		LogOut, Search, Box, PanelLeftClose, PanelLeft,
		Plus
	} from 'lucide-svelte';

	let { children } = $props();

	let user = $state<any>(null);
	let searchQuery = $state('');
	let mounted = $state(false);
	let showTerminal = $state(false);
	let sidebarOpen = $state(true);
	let terminalInput = $state('');
	let terminalOutput = $state<string[]>([]);
	let cmdHistory = $state<string[]>([]);
	let historyIdx = $state(-1);

	const API = 'http://localhost:8080/api';

	function getToken(): string | null {
		if (typeof localStorage !== 'undefined') return localStorage.getItem('forge_token');
		return null;
	}

	async function fetchUser() {
		const token = getToken();
		if (!token) return;
		try {
			const res = await fetch(`${API}/users/me`, { headers: { Authorization: `Bearer ${token}` } });
			if (res.ok) user = await res.json();
		} catch {}
	}

	function logout() {
		localStorage.removeItem('forge_token');
		window.location.href = '/auth';
	}

	async function runCommand(cmd: string) {
		if (!cmd.trim()) return;
		terminalOutput = [...terminalOutput, `$ ${cmd}`];
		cmdHistory = [...cmdHistory, cmd];
		historyIdx = cmdHistory.length;
		terminalInput = '';
		try {
			const res = await fetch(`${API}/exec`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${getToken()}` },
				body: JSON.stringify({ command: cmd })
			});
			if (res.ok) {
				const data = await res.json();
				if (data.stdout) terminalOutput = [...terminalOutput, ...data.stdout.split('\n').filter(Boolean)];
				if (data.stderr) terminalOutput = [...terminalOutput, ...data.stderr.split('\n').filter(Boolean)];
			} else terminalOutput = [...terminalOutput, `Error: ${res.status}`];
		} catch { terminalOutput = [...terminalOutput, 'Error: connection failed']; }
	}

	function onTerminalKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') runCommand(terminalInput);
		else if (e.key === 'ArrowUp') { e.preventDefault(); if (historyIdx > 0) { historyIdx--; terminalInput = cmdHistory[historyIdx]; } }
		else if (e.key === 'ArrowDown') { e.preventDefault(); if (historyIdx < cmdHistory.length - 1) { historyIdx++; terminalInput = cmdHistory[historyIdx]; } else { historyIdx = cmdHistory.length; terminalInput = ''; } }
	}

	onMount(() => { mounted = true; fetchUser(); });

	let isAuthPage = $derived($page.url.pathname === '/auth');
	let activeNav = $derived($page.url.pathname === '/' ? 'projects' : '');
</script>

{#if mounted}
	{#if isAuthPage}
		{@render children()}
	{:else}
		<div class="h-screen flex overflow-hidden bg-surface-950">
			<!-- Sidebar — orange glow, no gray -->
			<aside
				class="flex flex-col shrink-0 relative transition-all duration-300 overflow-hidden"
				class:w-56={sidebarOpen}
				class:w-0={!sidebarOpen}
			>
				<div class="w-56 h-full flex flex-col bg-forge-500/[0.04] border-r border-forge-500/10">
					<!-- Logo row + collapse -->
					<div class="flex items-center justify-between px-3 h-12 shrink-0">
						<div class="flex items-center gap-2 min-w-0">
							<div class="w-6 h-6 rounded-lg bg-gradient-to-br from-forge-400 via-forge-500 to-accent-500 flex items-center justify-center shrink-0 shadow-md shadow-forge-500/20">
								<FileCode2 size={11} class="text-white" />
							</div>
							{#if sidebarOpen}
								<div class="min-w-0">
									<h1 class="font-semibold text-surface-100 text-xs tracking-tight leading-tight">Forge</h1>
									<p class="text-[8px] text-forge-500/60 leading-tight">by Veloro</p>
								</div>
							{/if}
						</div>
						<button onclick={() => sidebarOpen = !sidebarOpen}
							class="text-surface-500 hover:text-forge-400 transition-colors p-1 rounded hover:bg-forge-500/10 shrink-0">
							<PanelLeftClose size={14} />
						</button>
					</div>

					<!-- Nav -->
					<nav class="flex-1 overflow-y-auto px-2 py-1 space-y-0.5">
						<a href="/"
							class="flex items-center gap-2.5 px-3 py-2 rounded-lg text-xs transition-all duration-300 shadow-lg {activeNav === 'projects' ? 'bg-forge-500/10 text-forge-400 shadow-forge-500/5' : 'text-surface-400 shadow-transparent hover:bg-forge-500/[0.06] hover:text-surface-200'}">
							<Box size={14} class="shrink-0" />
							Projects
						</a>
						<button onclick={() => showTerminal = !showTerminal}
							class="flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-xs transition-all duration-300 {showTerminal ? 'bg-forge-500/10 text-forge-400 shadow-lg shadow-forge-500/5' : 'text-surface-500 hover:bg-forge-500/[0.06] hover:text-surface-300'}">
							<TerminalIcon size={14} class="shrink-0" />
							Console
							<kbd class="ml-auto text-[8px] px-1 py-0.5 rounded bg-forge-500/10 text-forge-500/50">Ctrl+`</kbd>
						</button>
					</nav>

					<!-- Bottom -->
					<div class="px-2 pb-2 space-y-0.5 border-t border-forge-500/5 pt-2">
						{#if user}
							<div class="flex items-center gap-2 px-3 py-2 text-xs text-forge-400/60">
								<div class="w-4 h-4 rounded-full bg-forge-500/20 flex items-center justify-center text-[7px] font-bold text-forge-400 shrink-0">
									{user.username[0].toUpperCase()}
								</div>
								<span class="truncate">{user.username}</span>
							</div>
						{/if}
						<button onclick={logout}
							class="flex items-center gap-2 w-full px-3 py-2 rounded-lg text-xs text-surface-500 hover:bg-red-500/5 hover:text-red-400 transition-all duration-200">
							<LogOut size={14} /> Logout
						</button>
					</div>
				</div>
			</aside>

			<!-- Main: single orange-tinted glass surface -->
			<div class="flex-1 flex flex-col min-w-0 bg-gradient-to-br from-forge-500/[0.02] to-transparent">
				<div class="flex-1 flex flex-col min-h-0 px-2 pt-2 pb-2">
					<!-- Content card -->
					<div class="flex-1 flex flex-col min-h-0 bg-surface-900/30 backdrop-blur-md rounded-2xl overflow-hidden
								border border-forge-500/10 shadow-2xl shadow-black/20">
						{@render children()}
					</div>

					<!-- Terminal -->
					{#if showTerminal}
						<div class="mt-2 bg-surface-950/95 backdrop-blur-xl rounded-xl border border-forge-500/10 overflow-hidden font-mono text-xs"
							 transition:fade={{ duration: 120 }}>
							<div class="flex items-center justify-between px-3 py-1.5 bg-forge-500/[0.03] border-b border-forge-500/5">
								<div class="flex items-center gap-1.5 text-surface-500">
									<TerminalIcon size={10} class="text-forge-400" />
									<span class="text-[10px]">Console</span>
								</div>
								<button onclick={() => showTerminal = false}
									class="text-surface-600 hover:text-surface-300 transition-colors text-[10px] px-1">✕</button>
							</div>
							<div class="h-32 overflow-y-auto p-2.5 space-y-0.5 text-[11px] leading-relaxed">
								{#each terminalOutput as line}
									<div class:text-forge-400={line.startsWith('$')} class:text-red-400={line.startsWith('Error')} class:text-surface-500={!line.startsWith('$') && !line.startsWith('Error')}>
										{line || ' '}
									</div>
								{/each}
							</div>
							<div class="flex items-center gap-1.5 px-3 py-1.5 border-t border-forge-500/5 bg-forge-500/[0.02]">
								<span class="text-forge-400 shrink-0 text-[11px]">$</span>
								<input type="text" bind:value={terminalInput} placeholder="Type a command..."
									onkeydown={onTerminalKeydown}
									class="flex-1 bg-transparent text-surface-300 placeholder:text-surface-600 focus:outline-none text-[11px]" />
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
{/if}

<svelte:window onkeydown={(e) => {
	if (e.key === '`' && (e.ctrlKey || e.metaKey)) { e.preventDefault(); showTerminal = !showTerminal; }
	if (e.key === '\\' && (e.ctrlKey || e.metaKey)) { e.preventDefault(); sidebarOpen = !sidebarOpen; }
}} />