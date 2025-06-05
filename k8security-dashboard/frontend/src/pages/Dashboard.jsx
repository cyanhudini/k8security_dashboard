import { useEffect, useState } from 'react'
import { getVulnerabilities } from '../lib/api'
import FilterBar from '../components/SeverityFilter'
import EmailBar from '../components/EmailBar'
import '../styles/Dashboard.css'

export default function Dashboard() {
    const [vulns, setVulns] = useState([])
    
    useEffect(() => {
        getVulnerabilities().then(setVulns)
        console.log("Vulnerabilities fetched:", vulns)
    }, [])

    return (
        <div className="p-6">
            <h1>Kubernetes Security Dashboard</h1>
            <EmailBar/>
            <div className="dashboard" >
                <FilterBar onFilter={setVulns} />
                
                <table className=" ">
                    <thead>
                        <tr>
                            <th className="p-3">CVE</th>
                            <th className="p-3">Title</th>
                            <th className="p-3">installed version</th>
                            <th className="p-3">Severity</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y">
                        {vulns.map((vulns) => (
                            <tr>
                                <td >
                                    <input type="checkbox"></input>
                                </td>
                                <td className="p-3">{vulns.vuln_id}</td>
                                <td className="p-3">{vulns.pkg_name}</td>
                                <td className="p-3">{vulns.installed_version}</td>
                                <td className="p-3">{vulns.severity}</td>
                                <td className="p-3">{vulns.origin}</td>
                                <td className="p-3">{ vulns.scan_type}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}