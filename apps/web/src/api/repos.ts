export interface Repo {
	id: string;
	name: string;
	description: string | null;
	visibility: string;
	clone_url: string;
	created_at: string | null;
}

export async function listRepos(): Promise<Repo[]> {
	const res = await fetch('http://localhost:8080/api/repos/list');
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function getRepo(id: string): Promise<Repo> {
	const res = await fetch(`http://localhost:8080/api/repos/${id}`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function createRepo(data: {
	name: string;
	description?: string;
	visibility?: string;
}): Promise<Repo> {
	const token = localStorage.getItem('token');
	const res = await fetch('http://localhost:8080/api/repos/create', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Basic ${token}`
		},
		body: JSON.stringify(data)
	});
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export interface TreeEntry {
	mode: string;
	type: string;
	sha: string;
	name: string;
}

export async function browseTree(
	id: string,
	path = '',
	ref = 'HEAD'
): Promise<TreeEntry[]> {
	const params = new URLSearchParams();
	if (path) params.set('path', path);
	if (ref) params.set('ref', ref);
	const res = await fetch(`http://localhost:8080/api/repos/${id}/tree?${params}`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function updateVisibility(
	id: string,
	visibility: string
): Promise<void> {
	const token = localStorage.getItem('token');
	const res = await fetch(`http://localhost:8080/api/repos/${id}/visibility`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Basic ${token}`
		},
		body: JSON.stringify({ visibility })
	});
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
}
