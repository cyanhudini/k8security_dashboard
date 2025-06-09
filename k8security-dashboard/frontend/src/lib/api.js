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


export async function getGroupedVulnsByScanType(query) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/group_vulns_by_scan_type`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: query })
  });
  return await res.json();
}

export async function getGroupedVulnsByPkg(query) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/group_vulns_by_pkg`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: query })
  });
  return await res.json();
}


export async function postNewEmail(email) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/filter`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: email })
  })

  return await res.json()

}

export async function getReceiverEmails(){
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/receiver_emails`)

  return await res.json()
}


export async function addReceiverEmail(email) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/add_receiver_email`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email_adress: email })
  })
  return await res.json()
}

export async function setEmailStatus(email_id) {
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/set_email_status`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({email_id: email_id})
  })
  console.log(res)

  return await res.json()
}

export async function deleteVulnerabilities(vuln_ids) {
  console.log("Deleting vulnerabilities with IDs:", JSON.stringify(vuln_ids));
  
  const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}/delete_vulns`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(vuln_ids)
  })
  console.log("Response from deleteVulnerabilities:", res);
  if (!res.ok) {
    console.error("Failed to delete vulnerabilities:", res.statusText);
    throw new Error(`HTTP error! status: ${res.status}`);
  }

  return await res.json();
}