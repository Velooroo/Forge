<script lang="ts">
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { User, Key, Bell, Shield } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let email = $state('');

	onMount(() => {
		email = localStorage.getItem('email') || '';
	});
</script>

<div class="p-6 lg:p-8" in:fade={{ duration: 300, easing: cubicOut }}>
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight">Settings</h1>
		<p class="mt-1 text-sm text-white/40">Manage your account and preferences</p>
	</div>

	<div class="max-w-2xl space-y-6">
		<div class="rounded-2xl border border-white/10 bg-white/[0.04] p-6">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-emerald-400/10">
					<User class="h-5 w-5 text-emerald-400/60" />
				</div>
				<div>
					<h2 class="font-semibold">Profile</h2>
					<p class="text-sm text-white/40">{email}</p>
				</div>
			</div>
		</div>

		<div class="rounded-2xl border border-white/10 bg-white/[0.04] p-6 opacity-40">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-400/10">
					<Shield class="h-5 w-5 text-blue-400/60" />
				</div>
				<div>
					<h2 class="font-semibold">Access Tokens</h2>
					<p class="text-sm text-white/40">Coming soon</p>
				</div>
			</div>
		</div>

		<div class="rounded-2xl border border-white/10 bg-white/[0.04] p-6 opacity-40">
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-amber-400/10">
					<Bell class="h-5 w-5 text-amber-400/60" />
				</div>
				<div>
					<h2 class="font-semibold">Notifications</h2>
					<p class="text-sm text-white/40">Coming soon</p>
				</div>
			</div>
		</div>
	</div>
</div>
