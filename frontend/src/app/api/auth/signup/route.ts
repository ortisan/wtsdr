import { NextRequest } from "next/server";

export async function POST(req: NextRequest) {
  const body = await req.json().catch(() => ({}));
  const { email, password } = body || {};
  if (!email || !password) {
    return new Response(JSON.stringify({ error: "email and password are required" }), { status: 400 });
  }
  return new Response(JSON.stringify({ ok: true, userId: "dev-user-id", email }), {
    status: 201,
    headers: { "Content-Type": "application/json" },
  });
}
