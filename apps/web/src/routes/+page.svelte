<script lang="ts">
	import Button from '../components/ui/Button.svelte';
	import Input from '../components/ui/Input.svelte';
	import Label from '../components/ui/Label.svelte';
	import { Cpu, Github, ArrowRight, Sparkles, UserPlus } from 'lucide-svelte';
	import { login, register, isAuthenticated } from '../api/auth';
	import { fade, scale, type TransitionConfig } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	let email = $state('');
	let password = $state('');
	let username = $state('');
	let isRegister = $state(false);
	let authError = $state('');

	if (isAuthenticated()) {
		window.location.href = '/dashboard';
	}

	async function handleAuth(e: Event) {
		e.preventDefault();
		authError = '';
		try {
			if (isRegister) {
				await register(username, email, password);
			}
			await login(email, password);
			window.location.href = '/dashboard';
		} catch (err) {
			authError = 'Authentication failed. Check your credentials.';
		}
	}

	export function fadeScale(
		node: Element,
		{
			duration = 700,
			delay = 0,
			easing = (t: number) => t,
			start = 0.98,
			opacityStart = 0
		}: {
			duration?: number;
			delay?: number;
			easing?: (t: number) => number;
			start?: number;
			opacityStart?: number;
		} = {}
	): TransitionConfig {
		const style = getComputedStyle(node);
		const o0 = opacityStart;
		const o1 = +style.opacity || 1;
		const s0 = start;
		const s1 = 1;

		return {
			delay,
			duration,
			easing,
			css: (t) => {
				// t: 0→1 вход
				const opacity = o0 + (o1 - o0) * t;
				const scale = s0 + (s1 - s0) * t;
				return `opacity:${opacity};transform:scale(${scale});`;
			}
		};
	}
</script>

<div
	class="relative flex min-h-screen items-center justify-center overflow-hidden bg-[#030607] font-sans text-white selection:bg-emerald-500/30"
>
	<!-- AURORA / GLOW BACKGROUND -->
	<div class="absolute inset-0 z-0 overflow-hidden">
		<!-- Вместо Framer Motion используем CSS-анимацию через keyframes (простая, понятная) -->
		<div
			class="aurora-1 absolute top-[-15%] left-[-15%] h-[900px] w-[900px] rounded-full bg-emerald-500/20 blur-[140px]"
		/>
		<div
			class="aurora-2 absolute right-[-15%] bottom-[-15%] h-[900px] w-[900px] rounded-full bg-cyan-500/18 blur-[140px]"
		/>
		<div
			class="absolute top-[25%] right-[35%] h-[520px] w-[520px] rounded-full bg-blue-500/10 blur-[140px]"
		/>
	</div>

	<!-- SUBTLE GRID (masked) -->
	<div
		class="absolute inset-0 z-0 bg-[linear-gradient(rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.03)_1px,transparent_1px)] [mask-image:radial-gradient(ellipse_65%_60%_at_50%_45%,black,transparent)] bg-[size:72px_72px]"
	/>

	<div class="z-10 grid w-full max-w-6xl gap-20 px-6 lg:grid-cols-2">
		<!-- LEFT -->
		<div class="hidden flex-col justify-center space-y-10 lg:flex">
			<div in:fadeScale={{ duration: 700, easing: cubicOut, start: 0.98 }}>
				<div
					class="mb-6 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs font-medium text-white/80 backdrop-blur"
				>
					<Sparkles class="h-3 w-3 text-emerald-300" />
					<span>System Online</span>
					<span
						class="ml-1 inline-flex h-2 w-2 rounded-full bg-emerald-400 shadow-[0_0_14px_rgba(52,211,153,0.8)]"
					/>
				</div>

				<h1 class="text-6xl leading-[1.05] font-bold tracking-tight">
					Deployment
					<span
						class="bg-gradient-to-r from-emerald-300 via-cyan-300 to-blue-300 bg-clip-text text-transparent"
					>
						for hardware.
					</span>
				</h1>

				<p class="mt-6 max-w-md text-lg leading-relaxed text-white/55">
					Forge is your Git control plane with Spark integration for fleets. Soft UI, hard
					infrastructure.
				</p>
			</div>

			<div class="grid max-w-md grid-cols-2 gap-4">
				{#each [{ label: 'Uptime', val: '99.99%' }, { label: 'Deploy', val: '< 50ms' }] as item, i}
					<div
						class="rounded-2xl border border-white/10 bg-white/5 p-4 backdrop-blur-md"
						in:fadeScale={{ duration: 700, easing: cubicOut, start: 0.98 }}
					>
						<div class="text-sm text-white/45">{item.label}</div>
						<div class="mt-1 text-xl font-semibold">{item.val}</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- RIGHT -->
		<div class="flex items-center justify-center">
			<div
				class="group relative w-full max-w-[420px]"
				in:fadeScale={{ duration: 700, easing: cubicOut, start: 0.98 }}
			>
				<!-- BORDER GLOW (emerald -> cyan -> blue) -->
				<div
					class="absolute -inset-[1px] rounded-3xl opacity-70 blur-md transition duration-700 group-hover:opacity-100"
				/>

				<!-- INNER GLASS CARD -->
				<div
					class="relative rounded-3xl border border-white/10 bg-white/[0.06] p-8 shadow-2xl backdrop-blur-xl"
				>
					<div class="mb-8 flex flex-col items-center">
						<div
							class="mb-4 flex h-12 w-12 items-center justify-center rounded-2xl
              bg-gradient-to-tr from-emerald-400/80 to-cyan-400/80 shadow-lg shadow-emerald-500/15"
						>
							<Cpu class="h-6 w-6 text-white/80" />
						</div>
						<h2 class="text-2xl font-bold">Welcome Back</h2>
						<p class="mt-2 text-sm text-white/45">Sign in to access your console</p>
					</div>

					<form class="space-y-5" onsubmit={handleAuth}>
						{#if isRegister}
							<div class="space-y-2">
								<Label>Username</Label>
								<Input
									placeholder="you"
									bind:value={username}
									class_el="w-[70%]"
								/>
							</div>
						{/if}
						<div class="space-y-2">
							<Label>Email</Label>
							<Input
								placeholder="you@forge.dev"
								bind:value={email}
								class_el="w-[80%]"
							/>
						</div>

						<div class="space-y-2">
							<Label>Password</Label>
							<Input
								type="password"
								placeholder="••••••••"
								bind:value={password}
								class_el="w-[70%]"
							/>
						</div>

						{#if authError}
							<p class="text-xs text-red-400">{authError}</p>
						{/if}

						<Button type="submit">
							{isRegister ? 'Create Account' : 'Sign In'}
							<ArrowRight class="ml-2 h-4 w-4" />
						</Button>
					</form>

					<div class="mt-4 text-center">
						<button
							onclick={() => (isRegister = !isRegister)}
							class="text-xs text-white/40 transition-colors hover:text-white/60"
						>
							{isRegister ? 'Already have an account? Sign in' : "Don't have an account? Register"}
						</button>
					</div>

					<div class="mt-8 border-t border-white/10 pt-6">
						<Button
							variant="outline"
						>
							<Github class="mr-2 h-4 w-4" />
							Continue with GitHub
						</Button>
					</div>

					<div class="mt-6 text-center">
						<p class="text-xs text-white/30">
							By signing in, you agree to our
							<a href="#" class="text-white/60 underline hover:text-white">Terms</a>
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* Простые понятные keyframes для “ауры”, аналог твоего motion.div */
	@keyframes auroraMove1 {
		0% {
			transform: translate(0, 0) scale(1);
			opacity: 0.18;
		}
		50% {
			transform: translate(50px, -30px) scale(1.2);
			opacity: 0.32;
		}
		100% {
			transform: translate(0, 0) scale(1);
			opacity: 0.18;
		}
	}
	@keyframes auroraMove2 {
		0% {
			transform: translate(0, 0) scale(1);
			opacity: 0.16;
		}
		50% {
			transform: translate(-40px, 40px) scale(1.15);
			opacity: 0.34;
		}
		100% {
			transform: translate(0, 0) scale(1);
			opacity: 0.16;
		}
	}
	.aurora-1 {
		animation: auroraMove1 10s ease-in-out infinite;
	}
	.aurora-2 {
		animation: auroraMove2 12s ease-in-out infinite 1s;
	}
</style>
