import { browser } from '$app/environment';

function getStorage(key: string): string | null {
	if (!browser) return null;
	return localStorage.getItem(key);
}

function setStorage(key: string, value: string) {
	if (!browser) return;
	localStorage.setItem(key, value);
}

function removeStorage(key: string) {
	if (!browser) return;
	localStorage.removeItem(key);
}

export async function login(email: string, password: string) {
	const res = await fetch('http://localhost:8080/api/auth/login', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email, password })
	});
	if (!res.ok) throw new Error(`Login failed: HTTP ${res.status}`);

	const token = btoa(`${email}:${password}`);
	setStorage('token', token);
	setStorage('email', email);
}

export async function register(
	username: string,
	email: string,
	password: string
) {
	const res = await fetch('http://localhost:8080/api/auth/register', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ username, email, password })
	});
	if (!res.ok) throw new Error(`Register failed: HTTP ${res.status}`);
}

export function logout() {
	removeStorage('token');
	removeStorage('email');
	if (browser) {
		window.location.href = '/';
	}
}

export function isAuthenticated(): boolean {
	return !!getStorage('token');
}

export function getAuthHeader(): string {
	const token = getStorage('token');
	return token ? `Basic ${token}` : '';
}

export { browser };
