"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
    { href: "/dashboard", label: "Consultas", icon: "📋" },
    { href: "/services", label: "Meus Serviços", icon: "🔧" },
    { href: "/services/new", label: "Novo Serviço", icon: "➕" },
];

export default function Sidebar() {
    const pathname = usePathname();

    if (pathname === "/login") return null;

    return (
        <aside className="w-64 border-r border-[var(--color-border)] bg-[var(--color-bg-secondary)] flex flex-col">
            <div className="p-6 border-b border-[var(--color-border)]">
                <h1 className="text-lg font-bold text-[var(--color-text)]">WTSDR</h1>
                <p className="text-xs text-[var(--color-text-secondary)] mt-1">
                    Painel do Prestador
                </p>
            </div>

            <nav className="flex-1 py-4">
                {navItems.map((item) => {
                    const isActive = pathname === item.href;
                    return (
                        <Link
                            key={item.href}
                            href={item.href}
                            className={`flex items-center gap-3 px-6 py-3 text-sm font-medium transition-colors ${isActive
                                    ? "text-[var(--color-primary)] bg-blue-50 border-r-2 border-[var(--color-primary)]"
                                    : "text-[var(--color-text-secondary)] hover:bg-gray-100"
                                }`}
                        >
                            <span>{item.icon}</span>
                            {item.label}
                        </Link>
                    );
                })}
            </nav>

            <div className="p-6 border-t border-[var(--color-border)]">
                <button className="text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-error)] transition-colors">
                    Sair
                </button>
            </div>
        </aside>
    );
}
