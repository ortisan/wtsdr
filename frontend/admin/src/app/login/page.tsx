"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { apiLogin } from "@/lib/api";

export default function LoginPage() {
    const router = useRouter();
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState("");
    const [loading, setLoading] = useState(false);

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault();
        setError("");
        setLoading(true);
        try {
            await apiLogin(email, password);
            router.push("/dashboard");
        } catch {
            setError("Email ou senha inválidos");
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="min-h-screen flex items-center justify-center bg-[var(--color-bg-secondary)]">
            <form onSubmit={handleLogin} className="card p-10 w-full max-w-md">
                <h1 className="text-2xl font-bold text-[var(--color-text)] mb-2 text-center">WTSDR</h1>
                <p className="text-sm text-[var(--color-text-secondary)] mb-8 text-center">
                    Acesse o painel do prestador de serviços
                </p>

                {error && (
                    <div className="mb-4 p-3 bg-red-50 text-[var(--color-error)] text-sm rounded">{error}</div>
                )}

                <div className="space-y-4">
                    <div>
                        <label className="label">Email</label>
                        <input
                            type="email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            className="input-field"
                            placeholder="seu@email.com"
                            required
                        />
                    </div>
                    <div>
                        <label className="label">Senha</label>
                        <input
                            type="password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            className="input-field"
                            placeholder="••••••••"
                            required
                        />
                    </div>
                </div>

                <button type="submit" disabled={loading} className="btn-primary w-full mt-6">
                    {loading ? "Entrando..." : "Entrar"}
                </button>

                <p className="text-xs text-[var(--color-text-secondary)] mt-6 text-center">
                    Seed login: ana@email.com / password123
                </p>
            </form>
        </div>
    );
}
