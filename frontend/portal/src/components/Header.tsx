import Link from "next/link";

export default function Header() {
    return (
        <header className="border-b border-[var(--color-border)] bg-white sticky top-0 z-50">
            <div className="max-w-7xl mx-auto px-6 py-3 flex items-center justify-between">
                <Link href="/" className="text-xl font-bold text-[var(--color-text)]">
                    WTSDR
                </Link>
                <nav className="flex items-center gap-6">
                    <Link
                        href="/"
                        className="text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text)] transition-colors"
                    >
                        Pesquisar
                    </Link>
                    <button className="btn-primary text-sm !py-2 !px-5">Entrar</button>
                </nav>
            </div>
        </header>
    );
}
