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

export async function apiLogin(email: string, password: string): Promise<AuthResponse> {
    const res = await fetch(`${API_BASE}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
    });
    if (!res.ok) throw new Error("Credenciais inválidas");
    return res.json();
}

export async function apiSignup(name: string, email: string, password: string): Promise<AuthResponse> {
    const res = await fetch(`${API_BASE}/auth/signup`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, email, password }),
    });
    if (!res.ok) throw new Error("Erro no cadastro");
    return res.json();
}

export async function apiListServices(q?: string, category?: string): Promise<ServiceListResponse> {
    const params = new URLSearchParams();
    if (q) params.set("q", q);
    if (category && category !== "Todos") params.set("category", category);
    const res = await fetch(`${API_BASE}/services?${params.toString()}`);
    if (!res.ok) throw new Error("Erro ao buscar serviços");
    return res.json();
}

export async function apiGetService(id: string): Promise<ServiceDto> {
    const res = await fetch(`${API_BASE}/services/${id}`);
    if (!res.ok) throw new Error("Serviço não encontrado");
    return res.json();
}
