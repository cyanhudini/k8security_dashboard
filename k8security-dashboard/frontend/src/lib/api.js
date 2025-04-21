export async function getVulnerabilities() {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/vulns`)

  return await res.json()
}

export async function filterVulns() {
    const postOptions = {
        method: 'POST',
        headers: {},
        body: JSON.stringify({filters : ""})
    }

    return (await fetch(`${import.meta.env.VITE_BACKEND_URL}/filter_vulns_by_severity`, postOptions)).json()

}