export default function DashboardPage() {
    // Mock data for incoming service requests
    const inquiries = [
        { id: "1", userName: "Ana Silva", service: "Encanamento", location: "São Paulo, SP", date: "22/02/2026", status: "Pendente" },
        { id: "2", userName: "Carlos Oliveira", service: "Elétrica Residencial", location: "Campinas, SP", date: "21/02/2026", status: "Aceito" },
        { id: "3", userName: "Mariana Santos", service: "Pintura", location: "São Paulo, SP", date: "20/02/2026", status: "Pendente" },
        { id: "4", userName: "Roberto Lima", service: "Desentupimento", location: "Guarulhos, SP", date: "19/02/2026", status: "Recusado" },
    ];

    const statusStyles: Record<string, string> = {
        Pendente: "bg-yellow-50 text-yellow-700",
        Aceito: "bg-green-50 text-green-700",
        Recusado: "bg-red-50 text-red-700",
    };

    return (
        <div>
            <div className="mb-8">
                <h2 className="text-xl font-semibold text-[var(--color-text)]">
                    Consultas Recebidas
                </h2>
                <p className="text-sm text-[var(--color-text-secondary)] mt-1">
                    Solicitações de serviço enviadas por usuários
                </p>
            </div>

            <div className="card overflow-hidden">
                <table className="w-full text-sm">
                    <thead>
                        <tr className="bg-[var(--color-bg-secondary)] border-b border-[var(--color-border)]">
                            <th className="text-left px-6 py-3 font-medium text-[var(--color-text-secondary)]">Usuário</th>
                            <th className="text-left px-6 py-3 font-medium text-[var(--color-text-secondary)]">Serviço</th>
                            <th className="text-left px-6 py-3 font-medium text-[var(--color-text-secondary)]">Localização</th>
                            <th className="text-left px-6 py-3 font-medium text-[var(--color-text-secondary)]">Data</th>
                            <th className="text-left px-6 py-3 font-medium text-[var(--color-text-secondary)]">Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        {inquiries.map((row) => (
                            <tr
                                key={row.id}
                                className="border-b border-[var(--color-border)] hover:bg-[var(--color-bg-secondary)] transition-colors cursor-pointer"
                            >
                                <td className="px-6 py-4 font-medium text-[var(--color-text)]">{row.userName}</td>
                                <td className="px-6 py-4 text-[var(--color-text-secondary)]">{row.service}</td>
                                <td className="px-6 py-4 text-[var(--color-text-secondary)]">{row.location}</td>
                                <td className="px-6 py-4 text-[var(--color-text-secondary)]">{row.date}</td>
                                <td className="px-6 py-4">
                                    <span className={`inline-block px-3 py-1 rounded-full text-xs font-medium ${statusStyles[row.status]}`}>
                                        {row.status}
                                    </span>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
