export async function getVulnerabilities() {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/vulns`)

  return await res.json()
}

export async function filterVulnerabilities(filter_query) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/filter`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: filter_query })
  })

  return await res.json()

}


export async function postNewEmail(email) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/filter`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: email })
  })

  return await res.json()

}