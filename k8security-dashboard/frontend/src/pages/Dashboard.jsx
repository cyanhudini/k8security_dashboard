import { useEffect, useState } from 'react'
import { getVulnerabilities } from '../lib/api'

export default function Dashboard() {
    const [vulns, setVulns] = useState([])
    useEffect(() => {
        getVulnerabilities().then(setVulns)
    }, [])

    return (
        <div className="p-6">
            <h1 className="text-2xl font-bold">Kubernetes Security Dashboard</h1>
            <div className="overflow-auto rounded-lg shadow border">
                <table className="min-w-full bg-white">
                    <thead className="bg-gray-100 text-left text-sm font-semibold text-gray-700">
                        <tr>
                            <th className="p-3">CVE</th>
                            <th className="p-3"> Severity</th>
                            <th className="p-3">Package</th>
                            <th className="p-3">Title</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y">
                        {vulns.map()}
                    </tbody>
                </table>
            </div>
        </div>
    )

}