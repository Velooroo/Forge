<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onNavigate } from '$app/navigation';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import {
		GitBranch, Plus, Terminal as TerminalIcon, FileCode2,
		LogOut, Search, Command, Box
	} from 'lucide-svelte';

	let { children } = $props();

	let user = $state<any>(null);
	let searchQuery = $state('');
	let mounted = $state(false);
	let showTerminal = $state(false);
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
			const res = await fetch(`${API}/users/me`, {
				headers: { Authorization: `Bearer ${token}` }
			});
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
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${getToken()}`
				},
				body: JSON.stringify({ command: cmd })
			});
			if (res.ok) {
				const data = await res.json();
				if (data.stdout) terminalOutput = [...terminalOutput, ...data.stdout.split('\n')];
				if (data.stderr) terminalOutput = [...terminalOutput, ...data.stderr.split('\n')];
			} else {
				terminalOutput = [...terminalOutput, `Error: command failed (${res.status})`];
			}
		} catch {
			terminalOutput = [...terminalOutput, 'Error: connection failed'];
		}
	}

	function onTerminalKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			runCommand(terminalInput);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			if (historyIdx > 0) {
				historyIdx--;
				terminalInput = cmdHistory[historyIdx];
			}
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			if (historyIdx < cmdHistory.length - 1) {
				historyIdx++;
				terminalInput = cmdHistory[historyIdx];
			} else {
				historyIdx = cmdHistory.length;
				terminalInput = '';
			}
		}
	}

	onMount(() => { mounted = true; fetchUser(); });

	let isAuthPage = $derived($page.url.pathname === '/auth');
</script>

{#if mounted}
	{#if isAuthPage}
		{@render children()}
	{:else}
		<div class="h-screen flex overflow-hidden bg-surface-950">
			<!-- Sidebar — glass, orange glow right edge -->
			<aside class="w-56 lg:w-60 flex flex-col shrink-0 relative
						  bg-surface-900/30 backdrop-blur-xl
						  before:absolute before:inset-y-2 before:right-0 before:w-px
						  before:bg-gradient-to-b before:from-transparent before:via-forge-500/40 before:to-transparent">
				<!-- Logo -->
				<div class="flex items-center gap-2.5 px-4 h-13 shrink-0">
					<div class="w-7 h-7 rounded-lg bg-gradient-to-br from-forge-400 via-forge-500 to-accent-500 flex items-center justify-center shadow-lg shadow-forge-500/15">
						<FileCode2 size={13} class="text-white" />
					</div>
					<div>
						<h1 class="font-semibold text-surface-100 text-sm tracking-tight leading-tight">Forge</h1>
						<p class="text-[9px] text-surface-500 leading-tight">by Veloro</p>
					</div>
				</div>

				<!-- Search -->
				<div class="px-3 pt-2.5 pb-2">
					<div class="relative">
						<Search size={12} class="absolute left-2.5 top-1/2 -translate-y-1/2 text-surface-500 pointer-events-none" />
						<input type="text" placeholder="Search repos..." bind:value={searchQuery}
							class="w-full pl-7 pr-2 py-1.5 bg-surface-900/50 rounded-lg text-xs text-surface-300 
								   placeholder:text-surface-600 focus:outline-none focus:bg-surface-900/80
								   transition-all duration-200" />
					</div>
				</div>

				<!-- Nav -->
				<nav class="flex-1 overflow-y-auto px-2.5 py-1 space-y-0.5">
					<a href="/"
						class="flex items-center gap-2.5 px-3 py-2 rounded-lg text-xs transition-all duration-200
							   {$page.url.pathname === '/' ? 'bg-forge-500/10 text-forge-400 shadow-sm shadow-forge-500/5' : 'text-surface-400 hover:bg-surface-800/30 hover:text-surface-200'}">
						<Box size={14} class="shrink-0" />
						<span>Projects</span>
					</a>
					<a href="/explore" onclick={(e) => e.preventDefault()}
						class="flex items-center gap-2.5 px-3 py-2 rounded-lg text-xs text-surface-500 
							   hover:bg-surface-800/30 hover:text-surface-300 transition-all duration-200">
						<Search size={14} class="shrink-0" />
						<span>Explore</span>
					</a>
				</nav>

				<!-- Bottom section -->
				<div class="px-2.5 pb-2.5 space-y-0.5">
					<!-- Terminal toggle -->
					<button onclick={() => showTerminal = !showTerminal}
						class="flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-xs transition-all duration-200 {showTerminal ? 'bg-forge-500/10 text-forge-400' : 'text-surface-500 hover:bg-surface-800/30 hover:text-surface-300'}">
						<TerminalIcon size={14} class="shrink-0" />
						<span>Console</span>
						<kbd class="ml-auto text-[9px] px-1 py-0.5 rounded bg-surface-800/50 text-surface-500">Ctrl+`</kbd>
					</button>

					<!-- User -->
					{#if user}
						<div class="flex items-center gap-2.5 px-3 py-2 text-xs text-surface-500">
							<div class="w-5 h-5 rounded-full bg-gradient-to-br from-forge-400 to-forge-600 flex items-center justify-center text-[9px] font-bold text-white shrink-0 shadow-sm">
								{user.username[0].toUpperCase()}
							</div>
							<span class="truncate">{user.username}</span>
						</div>
					{/if}

					<button onclick={logout}
						class="flex items-center gap-2.5 w-full px-3 py-2 rounded-lg text-xs text-surface-500 
							   hover:bg-red-500/5 hover:text-red-400 transition-all duration-200">
						<LogOut size={14} />
						<span>Logout</span>
					</button>
				</div>
			</aside>

			<!-- Main: single glass surface everywhere -->
			<div class="flex-1 flex flex-col min-w-0
						bg-surface-900/20 backdrop-blur-sm">
				<!-- Content area -->
				<div class="flex-1 flex flex-col min-h-0 px-1.5 pt-1.5">
					<!-- Page content -->
					<div class="flex-1 flex flex-col min-h-0 bg-surface-900/40 backdrop-blur-sm rounded-2xl overflow-hidden
								border border-surface-800/20 shadow-xl shadow-black/10">
						{@render children()}
					</div>

					<!-- Terminal -->
					{#if showTerminal}
						<div class="mt-1.5 mb-1.5 bg-surface-950/90 backdrop-blur-xl rounded-xl border border-surface-800/30 overflow-hidden
									font-mono text-xs"
							 transition:fade={{ duration: 150 }}>
							<div class="flex items-center justify-between px-3 py-2 bg-surface-900/50 border-b border-surface-800/20">
								<div class="flex items-center gap-2 text-surface-400">
									<TerminalIcon size={12} class="text-forge-400" />
									<span class="text-[11px]">Console</span>
								</div>
								<button onclick={() => showTerminal = false}
									class="text-surface-600 hover:text-surface-300 transition-colors text-[11px] px-1.5">✕</button>
							</div>
							<div class="h-40 overflow-y-auto p-3 space-y-1 text-[12px] leading-relaxed" style="scroll-behavior: smooth;">
								{#each terminalOutput as line}
									<div class:text-forge-400={line.startsWith('$')}
										 class:text-red-400={line.startsWith('Error')}
										 class:text-surface-400={!line.startsWith('$') && !line.startsWith('Error')}>
										{line || ' '}
									</div>
								{/each}
							</div>
							<div class="flex items-center gap-2 px-3 py-2 border-t border-surface-800/20 bg-surface-900/30">
								<span class="text-forge-400 shrink-0">$</span>
								<input type="text" bind:value={terminalInput} placeholder="Type a command..."
									onkeydown={onTerminalKeydown}
									class="flex-1 bg-transparent text-surface-200 placeholder:text-surface-600 focus:outline-none text-[12px]" />
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
{/if}

<!-- Ctrl+` to toggle terminal -->
<svelte:window onkeydown={(e) => {
	if (e.key === '`' && (e.ctrlKey || e.metaKey)) {
		e.preventDefault();
		showTerminal = !showTerminal;
	}
}} />