import Link from "next/link";

export default function ServicesPage() {
    // Mock data for provider's services
    const services = [
        {
            id: "1",
            businessName: "Encanamentos Silva",
            services: ["Encanamento", "Desentupimento", "Caça-vazamentos"],
            status: "Ativo",
            inquiries: 12,
        },
        {
            id: "2",
            businessName: "Elétrica Express",
            services: ["Instalação Elétrica", "Manutenção Predial"],
            status: "Ativo",
            inquiries: 5,
        },
    ];

    return (
        <div>
            <div className="flex items-center justify-between mb-8">
                <div>
                    <h2 className="text-xl font-semibold text-[var(--color-text)]">
                        Meus Serviços
                    </h2>
                    <p className="text-sm text-[var(--color-text-secondary)] mt-1">
                        Gerencie os serviços cadastrados
                    </p>
                </div>
                <Link href="/services/new" className="btn-primary">
                    + Novo Serviço
                </Link>
            </div>

            <div className="grid gap-4">
                {services.map((svc) => (
                    <div key={svc.id} className="card p-6 flex items-start justify-between">
                        <div>
                            <h3 className="font-semibold text-[var(--color-text)]">
                                {svc.businessName}
                            </h3>
                            <div className="flex flex-wrap gap-2 mt-2">
                                {svc.services.map((s) => (
                                    <span
                                        key={s}
                                        className="bg-[var(--color-bg-secondary)] text-[var(--color-text-secondary)] text-xs px-3 py-1 rounded-full border border-[var(--color-border)]"
                                    >
                                        {s}
                                    </span>
                                ))}
                            </div>
                            <p className="text-xs text-[var(--color-text-secondary)] mt-3">
                                {svc.inquiries} consultas recebidas
                            </p>
                        </div>
                        <div className="flex items-center gap-3">
                            <span className="text-xs font-medium text-[var(--color-success)]">
                                {svc.status}
                            </span>
                            <button className="btn-outline text-xs !py-2 !px-4">Editar</button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
