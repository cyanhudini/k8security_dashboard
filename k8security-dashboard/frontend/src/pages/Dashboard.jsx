import { useEffect, useState } from 'react'
import { getVulnerabilities } from '../lib/api'
import FilterBar from '../components/FilterBar'
import '../styles/Dashboard.css'

export default function Dashboard() {
    const [vulns, setVulns] = useState([])
    useEffect(() => {
        getVulnerabilities().then(setVulns)
    }, [])

    return (
        <div className="p-6">
            <h1>Kubernetes Security Dashboard</h1>
            
            <div className="dashboard" >
                <FilterBar/>
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
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}