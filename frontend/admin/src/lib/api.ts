const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api/v1";

export interface AuthResponse {
    token: string;
    user_id: string;
    name: string;
    email: string;
}

export interface ServiceDto {
    id: string;
    business_name: string;
    description: string;
    services_provided: string[];
    rating: number;
    reviews: number;
    photos: string[];
    videos: string[];
    location: string;
    operating_hours: Record<string, { open: string; close: string } | null>;
}

export interface ServiceListResponse {
    data: ServiceDto[];
    total: number;
}

function getToken(): string | null {
    if (typeof window === "undefined") return null;
    return localStorage.getItem("token");
}

async function authFetch(url: string, options: RequestInit = {}): Promise<Response> {
    const token = getToken();
    return fetch(url, {
        ...options,
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
            ...options.headers,
        },
    });
}

export async function apiLogin(email: string, password: string): Promise<AuthResponse> {
    const res = await fetch(`${API_BASE}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
    });
    if (!res.ok) throw new Error("Credenciais inválidas");
    const data = await res.json();
    localStorage.setItem("token", data.token);
    localStorage.setItem("user", JSON.stringify(data));
    return data;
}

export async function apiSignup(name: string, email: string, password: string): Promise<AuthResponse> {
    const res = await fetch(`${API_BASE}/auth/signup`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, email, password }),
    });
    if (!res.ok) throw new Error("Erro no cadastro");
    const data = await res.json();
    localStorage.setItem("token", data.token);
    localStorage.setItem("user", JSON.stringify(data));
    return data;
}

export async function apiListServices(q?: string, category?: string): Promise<ServiceListResponse> {
    const params = new URLSearchParams();
    if (q) params.set("q", q);
    if (category && category !== "Todos") params.set("category", category);
    const res = await authFetch(`${API_BASE}/services?${params.toString()}`);
    if (!res.ok) throw new Error("Erro ao buscar serviços");
    return res.json();
}

export async function apiGetService(id: string): Promise<ServiceDto> {
    const res = await authFetch(`${API_BASE}/services/${id}`);
    if (!res.ok) throw new Error("Serviço não encontrado");
    return res.json();
}

export function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
    window.location.href = "/login";
}
