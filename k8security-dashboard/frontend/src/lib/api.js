export async function getVulnerabilities() {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/vulns`)

  return await res.json()
}