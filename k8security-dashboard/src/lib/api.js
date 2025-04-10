export async function getVulnerabilities() {
  const res = await fetch('')
  return await res.json()
}