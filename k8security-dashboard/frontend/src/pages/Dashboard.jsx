import Sidebar from '../components/Sidebar';
import '../styles/Dashboard.css';
import { useEffect, useState } from 'react';
import { getVulnerabilities } from '../lib/api';

export default function Dashboard() {
    const [vulns, setVulns] = useState([]);
    const [selectedVulns, setSelectedVulns] = useState([]);

    const toggleRow = (id) => {
        setSelectedVulns((prev) => {
            const isSelected = prev.some((v) => v.id === id);
            return isSelected
                ? prev.filter((v) => v.id !== id)
                : [...prev, { id }];
        });
        console.log("Toggled row:", id);
    };

    const handleDelete = (toDelete) => {
        console.log(toDelete);
        setVulns((prev) => prev.filter((v) => !toDelete.some((td) => td.id === v.id)));
        setSelectedVulns([]);
    };

    useEffect(() => {
        getVulnerabilities().then(setVulns);
        console.log("Vulnerabilities fetched:", vulns);
    }, []);

    return (
        <div className="dashboard-container mx-auto flex flex-col">
           <h1 className="text-2xl font-bold mb-4 text-center ">Kubernetes Security Dashboard</h1>
            <div className="dashboard w-full">
                <Sidebar onFilter={setVulns} selected={selectedVulns} onDelete={handleDelete} />
                <div>
                <table className="border-solid border-2 border-gray-300 w-full table-fixed">
                    <thead className="bg-emerald-900">
                        <tr>
                            <th className="p-3">Select</th>
                            <th className="p-3">CVE</th>
                            <th className="p-3">Pkg Name</th>
                            <th className="p-3">Installed Version</th>
                            <th className="p-3">Severity</th>
                            <th className="p-3">Origin</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y">
                        {vulns.map((vuln) => (
                            <tr
                                className="h-12 hover:bg-cyan-950 cursor-pointer"
                                key={vuln.id}
                                onClick={() => {
                                    console.log("Row clicked:", vuln.id);
                                    toggleRow(vuln.id);
                                }}
                            >
                                <td>
                                    <input
                                        type="checkbox"
                                        checked={selectedVulns.some((v) => v.id === vuln.id)}
                                        onChange={() => toggleRow(vuln.id)}
                                        onClick={(e) => e.stopPropagation()}
                                    />
                                </td>
                                <td className="p-3 truncate">{vuln.vuln_id}</td>
                                <td className="p-3 truncate">{vuln.pkg_name}</td>
                                <td className="p-3 truncate">{vuln.installed_version}</td>
                                <td className="p-3 truncate">{vuln.severity}</td>
                                <td className="p-3 truncate">{vuln.origin}</td>
                            </tr>
                        ))}
                    </tbody>
                    </table>
                    </div>
            </div>
        </div>
    );
}
