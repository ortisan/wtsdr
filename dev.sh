#!/bin/bash
set -e

echo "🚀 WTSDR — Starting all services..."
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"

# 1. Docker (database)
echo -e "${BLUE}[1/4]${NC} Starting database (Docker Compose)..."
docker compose -f "$ROOT_DIR/docker-compose.yml" up -d
echo -e "${GREEN}  ✓ Database running on localhost:5432${NC}"
echo ""

# 2. Backend
echo -e "${BLUE}[2/4]${NC} Starting backend (Rust/Actix)..."
cd "$ROOT_DIR/backend"
cargo run &
BACKEND_PID=$!
echo -e "${GREEN}  ✓ Backend starting on http://localhost:8080${NC}"
echo -e "${YELLOW}    Swagger: http://localhost:8080/swagger-ui/${NC}"
echo ""

# 3. Admin panel
echo -e "${BLUE}[3/4]${NC} Starting admin panel (Next.js)..."
cd "$ROOT_DIR/frontend/admin"
PORT=3001 npm run dev &
ADMIN_PID=$!
echo -e "${GREEN}  ✓ Admin panel on http://localhost:3001${NC}"
echo ""

# 4. Customer portal
echo -e "${BLUE}[4/4]${NC} Starting customer portal (Next.js)..."
cd "$ROOT_DIR/frontend/portal"
PORT=3000 npm run dev &
PORTAL_PID=$!
echo -e "${GREEN}  ✓ Customer portal on http://localhost:3000${NC}"
echo ""

echo "=========================================="
echo -e "${GREEN}All services started!${NC}"
echo ""
echo "  Backend API:     http://localhost:8080"
echo "  Swagger UI:      http://localhost:8080/swagger-ui/"
echo "  Admin Panel:     http://localhost:3001"
echo "  Customer Portal: http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop all services"
echo "=========================================="

# Trap to kill all background processes
cleanup() {
  echo ""
  echo "Shutting down..."
  kill $BACKEND_PID $ADMIN_PID $PORTAL_PID 2>/dev/null
  docker compose -f "$ROOT_DIR/docker-compose.yml" stop
  echo "Done."
}

trap cleanup EXIT INT TERM

wait
