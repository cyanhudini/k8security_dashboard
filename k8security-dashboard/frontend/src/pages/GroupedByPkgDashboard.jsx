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
        <div>
            {Object.entries(grouped).map(([key, group]) => (
            <div>
                <h2>
                PKG_NAME|PKG_ID({key})
                </h2>
                <ul>
                        {group.map(v => (
                        <div>
                        <li key={v.vuln_id}>
                            <span>
                                {v.id}  
                            </span> - 
                            {v.pkg_name}
                            {v.severity}
                            {v.vuln_id}
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