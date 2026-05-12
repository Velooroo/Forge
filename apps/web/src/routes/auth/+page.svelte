<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide, fly } from 'svelte/transition';
	import { FileCode2, Eye, EyeOff, Loader, ArrowRight, Sparkles } from 'lucide-svelte';

	let mode = $state<'login' | 'register'>('login');
	let loginValue = $state('');
	let username = $state('');
	let email = $state('');
	let password = $state('');
	let error = $state('');
	let loading = $state(false);
	let showPassword = $state(false);
	let mounted = $state(false);

	const API = 'http://localhost:8080/api';

	onMount(() => { mounted = true; });

	async function handleSubmit() {
		error = '';
		loading = true;
		try {
			if (mode === 'login') {
				const res = await fetch(`${API}/auth/login`, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ login: loginValue, password })
				});
				if (!res.ok) { error = 'Invalid credentials'; loading = false; return; }
				const data = await res.json();
				localStorage.setItem('forge_token', data.token);
				window.location.href = '/';
			} else {
				const res = await fetch(`${API}/auth/register`, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ username, email, password })
				});
				if (!res.ok) { const text = await res.text(); error = text; loading = false; return; }
				const data = await res.json();
				localStorage.setItem('forge_token', data.token);
				window.location.href = '/';
			}
		} catch { error = 'Connection error'; }
		loading = false;
	}

	function switchMode(m: 'login' | 'register') {
		if (m === mode) return;
		mode = m;
		error = '';
	}
</script>

{#if mounted}
<div class="min-h-screen flex items-center justify-center bg-surface-950 relative overflow-hidden">
	<!-- Aurora -->
	<div class="absolute inset-0 overflow-hidden pointer-events-none">
		<div class="absolute -top-40 -left-40 w-96 h-96 bg-forge-500/8 rounded-full blur-3xl animate-aurora" />
		<div class="absolute -bottom-40 -right-40 w-96 h-96 bg-accent-500/8 rounded-full blur-3xl animate-aurora" style="animation-delay: -3s" />
		<div class="absolute top-1/3 left-1/2 -translate-x-1/2 w-[500px] h-[500px] bg-forge-500/5 rounded-full blur-3xl animate-aurora" style="animation-delay: -6s" />
	</div>

	<!-- Grid -->
	<div class="absolute inset-0 opacity-[0.02]" style="background-image: linear-gradient(rgba(255,255,255,.08) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.08) 1px, transparent 1px); background-size: 40px 40px;" />

	<div class="relative w-full max-w-sm mx-4" transition:fade={{ duration: 400 }}>
		<!-- Logo -->
		<div class="text-center mb-8" in:fade={{ duration: 600, delay: 100 }}>
			<div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-forge-400 via-forge-500 to-accent-500 mb-4 shadow-lg shadow-forge-500/20">
				<FileCode2 size={26} class="text-white" />
			</div>
			<h1 class="text-2xl font-bold text-surface-100 tracking-tight">Forge</h1>
			<p class="text-sm text-surface-500 mt-1">by Veloro</p>
		</div>

		<!-- Card -->
		<div class="bg-surface-900/50 backdrop-blur-2xl border border-surface-700/40 rounded-2xl p-6 space-y-5" in:fade={{ duration: 400, delay: 200 }}>
			<!-- Tabs -->
			<div class="flex gap-1 bg-surface-950/50 rounded-lg p-1">
				<button
					onclick={() => switchMode('login')}
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all duration-300"
					class:bg-forge-500={mode === 'login'}
					class:text-white={mode === 'login'}
					class:text-surface-400={mode !== 'login'}
					class:hover:text-surface-200={mode !== 'login'}
				>
					Sign In
				</button>
				<button
					onclick={() => switchMode('register')}
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all duration-300"
					class:bg-forge-500={mode === 'register'}
					class:text-white={mode === 'register'}
					class:text-surface-400={mode !== 'register'}
					class:hover:text-surface-200={mode !== 'register'}
				>
					Register
				</button>
			</div>

			<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
				{#key mode}
					{#if mode === 'register'}
						<div in:fade={{ duration: 250, delay: 50 }} out:fade={{ duration: 150 }}>
							<label for="username" class="block text-xs font-medium text-surface-300 mb-1.5">Username</label>
							<input id="username" type="text" placeholder="your-username" bind:value={username} class="input-base text-sm" required />
						</div>
						<div in:fade={{ duration: 250, delay: 100 }} out:fade={{ duration: 150 }}>
							<label for="email" class="block text-xs font-medium text-surface-300 mb-1.5">Email</label>
							<input id="email" type="email" placeholder="you@example.com" bind:value={email} class="input-base text-sm" required />
						</div>
					{:else}
						<div in:fade={{ duration: 250, delay: 50 }} out:fade={{ duration: 150 }}>
							<label for="login" class="block text-xs font-medium text-surface-300 mb-1.5">Username or Email</label>
							<input id="login" type="text" placeholder="username or email" bind:value={loginValue} class="input-base text-sm" required />
						</div>
					{/if}
				{/key}

				<div>
					<label for="password" class="block text-xs font-medium text-surface-300 mb-1.5">Password</label>
					<div class="relative">
						<input id="password" type={showPassword ? 'text' : 'password'} placeholder="••••••••" bind:value={password} class="input-base text-sm pr-10" required />
						<button type="button" onclick={() => showPassword = !showPassword} class="absolute right-3 top-1/2 -translate-y-1/2 text-surface-400 hover:text-surface-200 transition-colors">
							{#if showPassword}<EyeOff size={16} />{:else}<Eye size={16} />{/if}
						</button>
					</div>
				</div>

				{#if error}
					<div class="text-sm text-red-400 bg-red-500/10 rounded-lg px-3 py-2 text-center" in:fade={{ duration: 200 }}>
						{error}
					</div>
				{/if}

				<button
					type="submit" disabled={loading}
					class="w-full py-2.5 bg-gradient-to-r from-forge-500 to-accent-500 hover:from-forge-400 hover:to-accent-400 
						   disabled:from-surface-700 disabled:to-surface-700 disabled:text-surface-500
						   text-white rounded-lg text-sm font-medium transition-all duration-300 
						   hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.97] 
						   flex items-center justify-center gap-2 group"
				>
					{#if loading}
						<Loader size={16} class="animate-spin" />
					{:else}
						<ArrowRight size={16} class="transition-transform duration-300 group-hover:translate-x-0.5" />
					{/if}
					{mode === 'login' ? 'Sign In' : 'Create Account'}
				</button>
			</form>

			<p class="text-xs text-surface-500 text-center">
				By continuing, you agree to the
				<a href="#" class="text-forge-400 hover:text-forge-300 transition-colors">Terms of Service</a>
			</p>
		</div>

		<!-- Features hint -->
		<div class="flex items-center justify-center gap-4 mt-6 text-xs text-surface-600" in:fade={{ duration: 600, delay: 500 }}>
			<span class="flex items-center gap-1"><Sparkles size={12} /> Self-hosted</span>
			<span class="flex items-center gap-1"><Sparkles size={12} /> Git hosting</span>
			<span class="flex items-center gap-1"><Sparkles size={12} /> Open source</span>
		</div>
	</div>
</div>
{/if}