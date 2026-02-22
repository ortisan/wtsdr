"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { apiListServices, type ServiceDto } from "@/lib/api";

const CATEGORIES = [
  "Todos",
  "Encanamento",
  "Elétrica",
  "Pintura",
  "Limpeza",
  "Jardinagem",
  "Reformas",
  "Mudanças",
  "Ar Condicionado",
];

function StarRating({ rating }: { rating: number }) {
  return (
    <span className="flex items-center gap-1">
      <span className="text-[var(--color-star)]">★</span>
      <span className="text-sm font-medium text-[var(--color-text)]">{rating}</span>
    </span>
  );
}

export default function HomePage() {
  const [searchQuery, setSearchQuery] = useState("");
  const [activeCategory, setActiveCategory] = useState("Todos");
  const [services, setServices] = useState<ServiceDto[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(true);
      apiListServices(searchQuery || undefined, activeCategory)
        .then((res) => setServices(res.data))
        .catch(() => setServices([]))
        .finally(() => setLoading(false));
    }, 300);
    return () => clearTimeout(timer);
  }, [searchQuery, activeCategory]);

  return (
    <div className="max-w-7xl mx-auto px-6">
      {/* Search Section */}
      <section className="py-10">
        <h1 className="text-2xl font-semibold text-center text-[var(--color-text)] mb-6">
          Encontre serviços perto de você
        </h1>
        <div className="max-w-2xl mx-auto relative">
          <svg
            className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-[var(--color-text-secondary)]"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            type="text"
            className="search-bar"
            placeholder="Pesquisar serviços, empresas..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </section>

      {/* Category Chips */}
      <section className="pb-6 flex flex-wrap gap-2">
        {CATEGORIES.map((cat) => (
          <button
            key={cat}
            onClick={() => setActiveCategory(cat)}
            className={`chip ${activeCategory === cat ? "chip-active" : ""}`}
          >
            {cat}
          </button>
        ))}
      </section>

      {/* Results */}
      <section className="pb-12">
        <p className="text-sm text-[var(--color-text-secondary)] mb-4">
          {loading ? "Buscando..." : `${services.length} resultado(s) encontrado(s)`}
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {services.map((svc) => (
            <Link key={svc.id} href={`/service/${svc.id}`}>
              <div className="card cursor-pointer">
                <div className="h-40 bg-[var(--color-bg-secondary)] flex items-center justify-center overflow-hidden">
                  {svc.photos.length > 0 ? (
                    <img src={svc.photos[0]} alt={svc.business_name} className="w-full h-full object-cover" />
                  ) : (
                    <span className="text-4xl">🏢</span>
                  )}
                </div>
                <div className="p-4">
                  <div className="flex items-start justify-between">
                    <h3 className="font-semibold text-[var(--color-text)] text-sm">{svc.business_name}</h3>
                    <StarRating rating={svc.rating} />
                  </div>
                  <p className="text-xs text-[var(--color-text-secondary)] mt-1">({svc.reviews} avaliações)</p>
                  <div className="flex flex-wrap gap-1 mt-3">
                    {svc.services_provided.slice(0, 3).map((s) => (
                      <span key={s} className="text-xs bg-[var(--color-bg-secondary)] text-[var(--color-text-secondary)] px-2 py-0.5 rounded">
                        {s}
                      </span>
                    ))}
                  </div>
                  <div className="flex items-center gap-3 mt-3 text-xs text-[var(--color-text-secondary)]">
                    <span>📍 {svc.location}</span>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>

        {!loading && services.length === 0 && (
          <div className="text-center py-16">
            <p className="text-lg text-[var(--color-text-secondary)]">Nenhum serviço encontrado.</p>
          </div>
        )}
      </section>
    </div>
  );
}
