	export async function login(email: string, password: string) {
		// В продакшене не хардкодим URL/ключи. Тут пример.
		try {
			const res = await fetch('http://localhost:8080/api/auth/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email, password })
			});
			if (!res.ok) throw new Error(`HTTP ${res.status}`);
			return await res.json();
		} catch (e) {
			console.error('Login failed:', e);
			throw e;
		}
	}
