import { useEffect, useState } from 'react'
import { getGroupedVulnsByPkg } from '../lib/api'

export default function GroupedByPkgDashboard() {
    const [grouped, setGrouped] = useState({})

    useEffect(() => {
        getGroupedVulnsByPkg().then(response => {
            console.log(response)
            setGrouped(response.vulnerabilities)
        })
    } ,[])

    return (
        <div className="p-4 space-y-6">
            
            {Object.entries(grouped).map(([key, group]) => (
            <div className = "border rounded p-4 grid grid-cols-3">
                <h2 className="font-bold text-lg col-span-1 break-all">{key}</h2>
                <ul className="col-span-2">
                        {group.map(v => (
                        <div>
                        <li key={v.vuln_id}>
                                    <p>{v.vuln_id} | {v.installed_version} | {v.severity} </p>
                        </li>
                             
                        </div>
                    ))}     
                </ul>
                <br></br>       
            </div>
            ))}
        </div>
    )

}