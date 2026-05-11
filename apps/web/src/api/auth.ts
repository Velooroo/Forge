export async function login(email: string, password: string) {
	const res = await fetch('http://localhost:8080/api/auth/login', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email, password })
	});
	if (!res.ok) throw new Error(`Login failed: HTTP ${res.status}`);

	const token = btoa(`${email}:${password}`);
	localStorage.setItem('token', token);
	localStorage.setItem('email', email);
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
	localStorage.removeItem('token');
	localStorage.removeItem('email');
	window.location.href = '/';
}

export function isAuthenticated(): boolean {
	return !!localStorage.getItem('token');
}

export function getAuthHeader(): string {
	return `Basic ${localStorage.getItem('token')}`;
}
