import Sidebar from '../components/Sidebar'
import '../styles/Dashboard.css'
import { useEffect, useState } from 'react'
import { getVulnerabilities } from '../lib/api'


export default function Dashboard() {
    const [vulns, setVulns] = useState([])
    const [selectedVulns, setSelectedVulns] = useState([])
    const toggleRow = (id) => {
        setSelectedVulns((prev) => prev.find((v) => v.id === id) ? prev.filter(v => v !== id) : [...prev, { id }])
        console.log("Toggled row:", id);
        console.log(selectedVulns)
    };
    
        useEffect(() => {
            getVulnerabilities().then(setVulns)
            console.log("Vulnerabilities fetched:", vulns)
        }, [])

    return (
        <div className="dashboard-container">
            <h1>Kubernetes Security Dashboard</h1>

            <div className="dashboard" >
                <Sidebar onFilter={setVulns} selected={selectedVulns} />
                
                <table className="border-solid border-2 border-gray-300 w-full ">
                    <thead className="bg-gray-200">
                        <tr>
                            <th className="p-3">Select</th>
                            <th className="p-3">CVE</th>
                            <th className="p-3">Pkg Name</th>
                            <th className="p-3">installed version</th>
                            <th className="p-3">Severity</th>
                            <th className="p-3">Origin</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y">
                        {vulns.map((vulns) => (
                            <tr className="h-12 hover:bg-cyan-950 cursor-pointer"
                                onClick={() => {
                                    console.log("Row clicked:", vulns.id);
                                    toggleRow(vulns.id)
                                }}>
                                <td >
                                    <input
                                        type="checkbox"
                                        checked={!!selectedVulns[vulns.id]}
                                        onChange={() => toggleRow(vulns.id)}
                                        onClick={(e) => e.stopPropagation()}
                                    ></input>
                                </td>
                                <td className="p-3 truncate">{vulns.vuln_id}</td>
                                <td className="p-3 truncate">{vulns.pkg_name}</td>
                                <td className="p-3 truncate">{vulns.installed_version}</td>
                                <td className="p-3 truncate">{vulns.severity}</td>
                                <td className="p-3 truncate">{vulns.origin}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}