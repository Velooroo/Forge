<script lang="ts">
	import { onMount } from 'svelte';
	import { FileCode2, Eye, EyeOff, Loader } from 'lucide-svelte';

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

				if (!res.ok) {
					error = 'Invalid credentials';
					loading = false;
					return;
				}

				const data = await res.json();
				localStorage.setItem('forge_token', data.token);
				window.location.href = '/';
			} else {
				const res = await fetch(`${API}/auth/register`, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ username, email, password })
				});

				if (!res.ok) {
					const text = await res.text();
					error = text;
					loading = false;
					return;
				}

				const data = await res.json();
				localStorage.setItem('forge_token', data.token);
				window.location.href = '/';
			}
		} catch {
			error = 'Connection error';
		}
		loading = false;
	}
</script>

{#if mounted}
<div class="min-h-screen flex items-center justify-center bg-surface-950 relative overflow-hidden">
	<!-- Aurora background -->
	<div class="absolute inset-0 overflow-hidden pointer-events-none">
		<div class="absolute -top-40 -left-40 w-96 h-96 bg-forge-500/10 rounded-full blur-3xl animate-aurora" />
		<div class="absolute -bottom-40 -right-40 w-96 h-96 bg-accent-500/10 rounded-full blur-3xl animate-aurora" style="animation-delay: -3s" />
		<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-forge-500/5 rounded-full blur-3xl animate-aurora" style="animation-delay: -6s" />
	</div>

	<!-- Grid pattern -->
	<div class="absolute inset-0 opacity-[0.03]" style="background-image: linear-gradient(rgba(255,255,255,.1) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.1) 1px, transparent 1px); background-size: 40px 40px;" />

	<div class="relative w-full max-w-sm mx-4 animate-fade-in">
		<!-- Logo -->
		<div class="text-center mb-8">
			<div class="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-gradient-to-br from-forge-400 to-accent-500 mb-4 shadow-lg shadow-forge-500/20">
				<FileCode2 size={28} class="text-white" />
			</div>
			<h1 class="text-2xl font-bold text-surface-100 tracking-tight">Forge</h1>
			<p class="text-sm text-surface-400 mt-1">by Veloro</p>
		</div>

		<!-- Card -->
		<div class="glass rounded-2xl p-6 space-y-5">
			<div class="flex gap-1 bg-surface-900/50 rounded-lg p-1">
				<button
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all duration-200"
					class:bg-surface-800={mode === 'login'}
					class:text-surface-100={mode === 'login'}
					class:text-surface-400={mode !== 'login'}
					onclick={() => { mode = 'login'; error = ''; }}
				>
					Sign In
				</button>
				<button
					class="flex-1 py-2 text-sm font-medium rounded-md transition-all duration-200"
					class:bg-surface-800={mode === 'register'}
					class:text-surface-100={mode === 'register'}
					class:text-surface-400={mode !== 'register'}
					onclick={() => { mode = 'register'; error = ''; }}
				>
					Register
				</button>
			</div>

			<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
				{#if mode === 'register'}
					<div class="animate-fade-in">
						<label for="username" class="block text-xs font-medium text-surface-300 mb-1.5">Username</label>
						<input
							id="username"
							type="text"
							placeholder="your-username"
							bind:value={username}
							class="input-base text-sm"
							required
						/>
					</div>
					<div class="animate-fade-in">
						<label for="email" class="block text-xs font-medium text-surface-300 mb-1.5">Email</label>
						<input
							id="email"
							type="email"
							placeholder="you@example.com"
							bind:value={email}
							class="input-base text-sm"
							required
						/>
					</div>
				{:else}
					<div class="animate-fade-in">
						<label for="login" class="block text-xs font-medium text-surface-300 mb-1.5">Username or Email</label>
						<input
							id="login"
							type="text"
							placeholder="username or email"
							bind:value={loginValue}
							class="input-base text-sm"
							required
						/>
					</div>
				{/if}

				<div class="animate-fade-in" style="animation-delay: 0.05s">
					<label for="password" class="block text-xs font-medium text-surface-300 mb-1.5">Password</label>
					<div class="relative">
						<input
							id="password"
							type={showPassword ? 'text' : 'password'}
							placeholder="••••••••"
							bind:value={password}
							class="input-base text-sm pr-10"
							required
						/>
						<button
							type="button"
							onclick={() => showPassword = !showPassword}
							class="absolute right-3 top-1/2 -translate-y-1/2 text-surface-400 hover:text-surface-200 transition-colors"
						>
							{#if showPassword}
								<EyeOff size={16} />
							{:else}
								<Eye size={16} />
							{/if}
						</button>
					</div>
				</div>

				{#if error}
					<div class="text-sm text-red-400 bg-red-500/10 rounded-lg px-3 py-2 animate-fade-in">
						{error}
					</div>
				{/if}

				<button
					type="submit"
					disabled={loading}
					class="w-full py-2.5 bg-forge-500 hover:bg-forge-400 disabled:bg-surface-700 disabled:text-surface-500 
						   text-white rounded-lg text-sm font-medium transition-all duration-200 
						   hover:shadow-lg hover:shadow-forge-500/25 active:scale-[0.98] 
						   flex items-center justify-center gap-2"
				>
					{#if loading}
						<Loader size={16} class="animate-spin" />
					{/if}
					{mode === 'login' ? 'Sign In' : 'Create Account'}
				</button>
			</form>

			<p class="text-xs text-surface-500 text-center">
				By continuing, you agree to the
				<a href="#" class="text-forge-400 hover:text-forge-300 transition-colors">Terms of Service</a>
			</p>
		</div>
	</div>
</div>
{/if}