import { NextRequest } from "next/server";

export async function POST(req: NextRequest) {
  const body = await req.json().catch(() => ({}));
  const { email } = body || {};
  // Placeholder implementation. Replace with real authentication.
  if (!email) {
    return new Response(JSON.stringify({ error: "email is required" }), { status: 400 });
  }
  return new Response(JSON.stringify({ ok: true, token: "dev-token", email }), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
}
