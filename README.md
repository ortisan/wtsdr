# WTSDR — Marketplace de Serviços Locais

Plataforma de marketplace que conecta **usuários** a **prestadores de serviço** na região, com busca por geolocalização (PostGIS), upload de mídia (S3/local), e pagamentos (Stripe — futuro).

## Arquitetura

```
wtsdr/
├── backend/          # Rust (Actix-web + Diesel + PostGIS)
├── frontend/
│   ├── admin/        # Next.js — Painel do Prestador (porta 3001)
│   └── portal/       # Next.js — Portal do Cliente (porta 3000)
├── docker-compose.yml
└── dev.sh            # Script para subir todos os serviços
```

### Backend (Clean Architecture)

| Camada | Diretório | Responsabilidade |
|--------|-----------|------------------|
| **Domain** | `src/domain/` | Entidades (`User`, `ProviderProfile`, `ServiceRequest`), erros |
| **Application** | `src/application/` | Use cases (`broadcast`, `create_provider`, `search_providers`), traits (`ProviderRepository`, `MediaStorage`) |
| **Infrastructure** | `src/infrastructure/` | Diesel/PostGIS repo, S3 adapter, Local storage adapter, Auth (Argon2 + JWT) |
| **Presentation** | `src/presentation/` | Actix-web controllers e rotas HTTP |

### Stack

- **Backend:** Rust, Actix-web, Diesel ORM, PostGIS, Argon2, JWT
- **Frontend:** Next.js 15 (App Router), TypeScript, Tailwind CSS, Zod, React Hook Form
- **Banco:** PostgreSQL 16 + PostGIS 3.4
- **Storage:** S3 (produção) / Disco local (desenvolvimento)
- **CI/CD:** GitHub Actions, Release Please, Conventional Commits

---

## Quick Start

### Pré-requisitos

- [Docker](https://docker.com) (para o banco de dados)
- [Rust](https://rustup.rs) (stable)
- [Node.js](https://nodejs.org) 20+
- [Diesel CLI](https://diesel.rs): `cargo install diesel_cli --no-default-features --features postgres`

### 1. Subir tudo com um comando

```bash
./dev.sh
```

Isso irá:
1. Subir o PostgreSQL + PostGIS via Docker
2. Iniciar o backend Rust na porta `8080`
3. Iniciar o Admin Panel na porta `3001`
4. Iniciar o Customer Portal na porta `3000`

### 2. Ou subir manualmente

```bash
# Banco de dados
docker compose up -d

# Backend
cd backend && cargo run

# Admin Panel
cd frontend/admin && npm run dev -- --port 3001

# Customer Portal
cd frontend/portal && npm run dev
```

---

## URLs Locais

| Serviço | URL |
|---------|-----|
| Backend API | http://localhost:8080 |
| Swagger UI | http://localhost:8080/swagger-ui/ |
| Health Check | http://localhost:8080/api/health |
| Admin Panel | http://localhost:3001 |
| Customer Portal | http://localhost:3000 |

---

## API Endpoints

### Auth

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/v1/auth/signup` | Cadastro com email/senha → retorna JWT |
| `POST` | `/api/v1/auth/login` | Login com email/senha → retorna JWT |

**Signup body:**
```json
{ "name": "João Silva", "email": "joao@email.com", "password": "senha123" }
```

**Login body:**
```json
{ "email": "joao@email.com", "password": "senha123" }
```

### Broadcast

| Método | Rota | Descrição |
|--------|------|-----------|
| `POST` | `/api/v1/broadcast` | Busca provedores em raio via PostGIS |

---

## Modelo de Dados

### Users
Tabela base de identidade (login). Um usuário pode ser cliente, prestador, ou ambos.

| Campo | Tipo |
|-------|------|
| `id` | UUID v7 |
| `oauth_sub` | VARCHAR (identificador do auth) |
| `name` | VARCHAR |
| `email` | VARCHAR (unique) |
| `password_hash` | VARCHAR (Argon2) |
| `created_at` | TIMESTAMPTZ |

### Provider Profiles
Perfil opcional 1:1 com `users`. Existe apenas se o usuário ativar o modo prestador.

| Campo | Tipo |
|-------|------|
| `id` | UUID v7 |
| `user_id` | UUID (FK → users) |
| `business_name` | VARCHAR |
| `description` | TEXT |
| `services_provided` | TEXT[] |
| `tags` | TEXT[] |
| `photos` | TEXT[] |
| `videos` | TEXT[] |
| `operating_hours` | JSONB |
| `ranking_score` | DOUBLE |
| `geom` | geometry(Point, 4326) |

---

## Variáveis de Ambiente

Configuradas em `backend/.env`:

| Variável | Default | Descrição |
|----------|---------|-----------|
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/marketplace` | URL do PostgreSQL |
| `STORAGE_MODE` | `local` | `local` ou `s3` |
| `LOCAL_STORAGE_PATH` | `./uploads` | Pasta para uploads em dev |
| `LOCAL_STORAGE_URL` | `http://localhost:8080/uploads` | URL pública dos uploads locais |
| `S3_BUCKET` | — | Nome do bucket S3 (produção) |
| `S3_REGION` | — | Região do S3 |
| `JWT_SECRET` | `dev-secret-change-in-production` | Chave secreta para tokens JWT |

---

## Seed Data

O Docker Compose executa `backend/scripts/seed.sql` automaticamente na primeira inicialização, criando:

- **7 usuários** (5 prestadores + 2 clientes)
- **5 perfis de prestadores** com coordenadas reais de São Paulo
- Senha padrão para todos os mocks: `password123`

Para resetar os dados:
```bash
docker compose down -v && docker compose up -d
```

---

## Frontend — Admin Panel (`frontend/admin`)

Painel para prestadores de serviço gerenciarem seus negócios.

| Página | Rota | Funcionalidade |
|--------|------|----------------|
| Login | `/login` | OAuth2 redirect |
| Dashboard | `/dashboard` | Consultas recebidas (tabela com status) |
| Meus Serviços | `/services` | Lista de serviços cadastrados |
| Novo Serviço | `/services/new` | Formulário com Zod validation, tag input, horários, upload de fotos/vídeos |

## Frontend — Customer Portal (`frontend/portal`)

Portal para clientes encontrarem serviços.

| Página | Rota | Funcionalidade |
|--------|------|----------------|
| Pesquisa | `/` | Busca + filtros por categoria + grid de cards |
| Detalhe | `/service/[id]` | Galeria, descrição, serviços, horários, CTA |

---

## Roadmap

- [x] Backend Rust com Clean Architecture
- [x] Diesel ORM + PostGIS migrations
- [x] Auth local (Argon2 + JWT)
- [x] Swagger UI (utoipa)
- [x] Media Storage (S3 + Local)
- [x] Admin Panel (Next.js)
- [x] Customer Portal (Next.js)
- [x] Docker Compose + Seed Data
- [x] CI/CD (GitHub Actions + Release Please)
- [ ] Integração real Frontend ↔ Backend API
- [ ] Stripe Payments
- [ ] Notificações ao Provider
- [ ] OAuth2 externo (Google, etc)
