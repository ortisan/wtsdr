-- Enable PostGIS
CREATE EXTENSION IF NOT EXISTS postgis;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    oauth_sub VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- Provider profiles table
CREATE TABLE IF NOT EXISTS provider_profiles (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    business_name VARCHAR(255) NOT NULL,
    description TEXT,
    services_provided TEXT[] NOT NULL DEFAULT '{}',
    tags TEXT[] NOT NULL DEFAULT '{}',
    photos TEXT[] NOT NULL DEFAULT '{}',
    videos TEXT[] NOT NULL DEFAULT '{}',
    operating_hours JSONB,
    ranking_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    geom geometry(Point, 4326) NOT NULL,
    CONSTRAINT idx_user_id_unique UNIQUE(user_id)
);

CREATE INDEX IF NOT EXISTS idx_provider_profiles_geom ON provider_profiles USING GIST (geom);
CREATE INDEX IF NOT EXISTS idx_provider_profiles_services ON provider_profiles USING GIN (services_provided);

-- ============================================================
-- SEED DATA
-- ============================================================

-- Users
-- password_hash below is argon2 hash of 'password123'
INSERT INTO users (id, oauth_sub, name, email, password_hash, created_at) VALUES
  ('019505a0-0001-7000-8000-000000000001', 'local|user1', 'Ana Silva', 'ana@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0002-7000-8000-000000000002', 'local|user2', 'Carlos Oliveira', 'carlos@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0003-7000-8000-000000000003', 'local|user3', 'Mariana Santos', 'mariana@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0004-7000-8000-000000000004', 'local|user4', 'Roberto Lima', 'roberto@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0005-7000-8000-000000000005', 'local|user5', 'Fernanda Costa', 'fernanda@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0006-7000-8000-000000000006', 'local|client1', 'João Pedro', 'joao@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW()),
  ('019505a0-0007-7000-8000-000000000007', 'local|client2', 'Lucia Mendes', 'lucia@email.com', '$argon2id$v=19$m=19456,t=2,p=1$c2VlZHNhbHQ$YhFKjFJEInCAlIJGS1Dl/HKk1E3EOgcvGKs5J7ID8pQ', NOW())
ON CONFLICT (id) DO NOTHING;

-- Provider Profiles (São Paulo region coordinates)
INSERT INTO provider_profiles (id, user_id, business_name, description, services_provided, tags, photos, videos, operating_hours, ranking_score, created_at, updated_at, geom) VALUES
(
  '019505b0-0001-7000-8000-000000000001',
  '019505a0-0001-7000-8000-000000000001',
  'Encanamentos Silva',
  'Mais de 15 anos de experiência em serviços hidráulicos residenciais e comerciais. Atendemos emergências 24h.',
  ARRAY['Encanamento', 'Desentupimento', 'Caça-vazamentos'],
  ARRAY['urgente', 'residencial', 'comercial'],
  ARRAY['https://placehold.co/600x400/1A73E8/white?text=Encanamentos+Silva'],
  ARRAY[]::TEXT[],
  '{"segunda": {"open": "08:00", "close": "18:00"}, "terca": {"open": "08:00", "close": "18:00"}, "quarta": {"open": "08:00", "close": "18:00"}, "quinta": {"open": "08:00", "close": "18:00"}, "sexta": {"open": "08:00", "close": "18:00"}, "sabado": {"open": "08:00", "close": "14:00"}, "domingo": null}',
  4.7, NOW(), NOW(),
  ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)
),
(
  '019505b0-0002-7000-8000-000000000002',
  '019505a0-0002-7000-8000-000000000002',
  'Elétrica Express',
  'Serviços elétricos rápidos e seguros para sua casa ou empresa. Equipe qualificada e certificada.',
  ARRAY['Elétrica', 'Instalação Elétrica', 'Manutenção Predial'],
  ARRAY['certificado', 'rápido', 'seguro'],
  ARRAY['https://placehold.co/600x400/34A853/white?text=Eletrica+Express'],
  ARRAY[]::TEXT[],
  '{"segunda": {"open": "07:00", "close": "19:00"}, "terca": {"open": "07:00", "close": "19:00"}, "quarta": {"open": "07:00", "close": "19:00"}, "quinta": {"open": "07:00", "close": "19:00"}, "sexta": {"open": "07:00", "close": "19:00"}, "sabado": {"open": "08:00", "close": "16:00"}, "domingo": null}',
  4.5, NOW(), NOW(),
  ST_SetSRID(ST_MakePoint(-47.0616, -22.9064), 4326)
),
(
  '019505b0-0003-7000-8000-000000000003',
  '019505a0-0003-7000-8000-000000000003',
  'Jardins & Paisagismo',
  'Transformamos espaços verdes com serviços completos de jardinagem e paisagismo. Projetos sob medida.',
  ARRAY['Jardinagem', 'Paisagismo', 'Poda de Árvores'],
  ARRAY['paisagismo', 'verde', 'sustentável'],
  ARRAY['https://placehold.co/600x400/34A853/white?text=Jardins+Paisagismo'],
  ARRAY['https://placehold.co/600x400/FBBC04/white?text=Video+Tour'],
  '{"segunda": {"open": "08:00", "close": "17:00"}, "terca": {"open": "08:00", "close": "17:00"}, "quarta": {"open": "08:00", "close": "17:00"}, "quinta": {"open": "08:00", "close": "17:00"}, "sexta": {"open": "08:00", "close": "17:00"}, "sabado": null, "domingo": null}',
  4.9, NOW(), NOW(),
  ST_SetSRID(ST_MakePoint(-46.6580, -23.5630), 4326)
),
(
  '019505b0-0004-7000-8000-000000000004',
  '019505a0-0004-7000-8000-000000000004',
  'PintaTop Pinturas',
  'Pinturas profissionais com acabamento impecável. Residências e comércios. Orçamento gratuito.',
  ARRAY['Pintura', 'Textura', 'Grafiato'],
  ARRAY['acabamento', 'orçamento grátis'],
  ARRAY['https://placehold.co/600x400/EA4335/white?text=PintaTop'],
  ARRAY[]::TEXT[],
  '{"segunda": {"open": "08:00", "close": "18:00"}, "terca": {"open": "08:00", "close": "18:00"}, "quarta": {"open": "08:00", "close": "18:00"}, "quinta": {"open": "08:00", "close": "18:00"}, "sexta": {"open": "08:00", "close": "18:00"}, "sabado": null, "domingo": null}',
  4.3, NOW(), NOW(),
  ST_SetSRID(ST_MakePoint(-46.5325, -23.4630), 4326)
),
(
  '019505b0-0005-7000-8000-000000000005',
  '019505a0-0005-7000-8000-000000000005',
  'Clima Perfeito',
  'Instalação, limpeza e manutenção de ar condicionado split e central com garantia de 1 ano.',
  ARRAY['Ar Condicionado', 'Manutenção de Ar', 'Instalação Split'],
  ARRAY['garantia', 'split', 'central'],
  ARRAY['https://placehold.co/600x400/1A73E8/white?text=Clima+Perfeito'],
  ARRAY[]::TEXT[],
  '{"segunda": {"open": "08:00", "close": "20:00"}, "terca": {"open": "08:00", "close": "20:00"}, "quarta": {"open": "08:00", "close": "20:00"}, "quinta": {"open": "08:00", "close": "20:00"}, "sexta": {"open": "08:00", "close": "20:00"}, "sabado": {"open": "09:00", "close": "15:00"}, "domingo": null}',
  4.6, NOW(), NOW(),
  ST_SetSRID(ST_MakePoint(-46.6250, -23.5330), 4326)
)
ON CONFLICT (id) DO NOTHING;
