import Link from "next/link";
import { apiGetService } from "@/lib/api";

const DAYS_PT: Record<string, string> = {
    segunda: "Segunda",
    terca: "Terça",
    quarta: "Quarta",
    quinta: "Quinta",
    sexta: "Sexta",
    sabado: "Sábado",
    domingo: "Domingo",
};

export default async function ServiceDetailPage({
    params,
}: {
    params: Promise<{ id: string }>;
}) {
    const { id } = await params;

    let svc;
    try {
        svc = await apiGetService(id);
    } catch {
        return (
            <div className="max-w-4xl mx-auto px-6 py-16 text-center">
                <h2 className="text-xl font-semibold text-[var(--color-text)]">Serviço não encontrado</h2>
                <Link href="/" className="text-[var(--color-primary)] mt-4 inline-block">← Voltar</Link>
            </div>
        );
    }

    const hoursEntries = Object.entries(svc.operating_hours || {});

    return (
        <div className="max-w-4xl mx-auto px-6 py-8">
            <nav className="text-sm text-[var(--color-text-secondary)] mb-6">
                <Link href="/" className="hover:text-[var(--color-primary)]">Pesquisar</Link>
                <span className="mx-2">›</span>
                <span className="text-[var(--color-text)]">{svc.business_name}</span>
            </nav>

            {/* Photo Gallery */}
            <div className="grid grid-cols-3 gap-2 mb-8 rounded-lg overflow-hidden">
                <div className="col-span-2 h-72 bg-[var(--color-bg-secondary)] flex items-center justify-center overflow-hidden">
                    {svc.photos.length > 0 ? (
                        <img src={svc.photos[0]} alt={svc.business_name} className="w-full h-full object-cover" />
                    ) : (
                        <span className="text-6xl">🏢</span>
                    )}
                </div>
                <div className="flex flex-col gap-2">
                    <div className="flex-1 bg-[var(--color-bg-secondary)] flex items-center justify-center overflow-hidden">
                        {svc.photos.length > 1 ? (
                            <img src={svc.photos[1]} alt="Foto" className="w-full h-full object-cover" />
                        ) : (
                            <span className="text-3xl">📷</span>
                        )}
                    </div>
                    <div className="flex-1 bg-[var(--color-bg-secondary)] flex items-center justify-center relative overflow-hidden">
                        {svc.videos.length > 0 ? (
                            <>
                                <img src={svc.videos[0]} alt="Vídeo" className="w-full h-full object-cover" />
                                <span className="absolute bottom-2 right-2 bg-black/70 text-white text-xs px-2 py-1 rounded">
                                    {svc.videos.length} vídeo(s)
                                </span>
                            </>
                        ) : (
                            <span className="text-3xl">🎬</span>
                        )}
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-3 gap-8">
                <div className="col-span-2 space-y-6">
                    <div>
                        <h1 className="text-2xl font-semibold text-[var(--color-text)]">{svc.business_name}</h1>
                        <div className="flex items-center gap-3 mt-2">
                            <span className="text-[var(--color-star)]">★</span>
                            <span className="text-sm font-medium">{svc.rating}</span>
                            <span className="text-sm text-[var(--color-text-secondary)]">({svc.reviews} avaliações)</span>
                            <span className="text-sm text-[var(--color-text-secondary)]">· 📍 {svc.location}</span>
                        </div>
                    </div>

                    <div>
                        <h2 className="text-sm font-semibold text-[var(--color-text)] mb-2">Sobre</h2>
                        <p className="text-sm text-[var(--color-text-secondary)] leading-relaxed">{svc.description}</p>
                    </div>

                    <div>
                        <h2 className="text-sm font-semibold text-[var(--color-text)] mb-3">Serviços Oferecidos</h2>
                        <div className="flex flex-wrap gap-2">
                            {svc.services_provided.map((s) => (
                                <span key={s} className="inline-block bg-blue-50 text-[var(--color-primary)] text-sm px-4 py-2 rounded-full font-medium">
                                    {s}
                                </span>
                            ))}
                        </div>
                    </div>

                    <div className="card p-6 bg-[var(--color-bg-secondary)] border-[var(--color-primary)] border-l-4">
                        <h3 className="font-semibold text-[var(--color-text)] mb-2">Interessado neste serviço?</h3>
                        <p className="text-sm text-[var(--color-text-secondary)] mb-4">Solicite um orçamento e o prestador entrará em contato.</p>
                        <button className="btn-primary">Solicitar Serviço</button>
                    </div>
                </div>

                <aside className="space-y-6">
                    <div className="card p-5">
                        <h3 className="text-sm font-semibold text-[var(--color-text)] mb-3">Horário de Funcionamento</h3>
                        <ul className="space-y-2">
                            {hoursEntries.map(([key, val]) => (
                                <li key={key} className="flex justify-between text-sm">
                                    <span className="text-[var(--color-text)]">{DAYS_PT[key] || key}</span>
                                    <span className={val === null ? "text-[var(--color-error)]" : "text-[var(--color-text-secondary)]"}>
                                        {val === null ? "Fechado" : `${val.open} - ${val.close}`}
                                    </span>
                                </li>
                            ))}
                        </ul>
                    </div>

                    <div className="card p-5">
                        <h3 className="text-sm font-semibold text-[var(--color-text)] mb-3">Localização</h3>
                        <div className="h-36 bg-[var(--color-bg-secondary)] rounded flex items-center justify-center">
                            <span className="text-2xl">🗺️</span>
                        </div>
                        <p className="text-sm text-[var(--color-text-secondary)] mt-3">📍 {svc.location}</p>
                    </div>
                </aside>
            </div>
        </div>
    );
}
