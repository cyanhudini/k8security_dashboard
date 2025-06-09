import { useState, useEffect } from 'react'
import { filterVulnerabilities } from '../lib/api'

const severities = ['LOW', 'MEDIUM', 'HIGH', 'CRITICAL']

export default function SeverityFilter({ onFilter }) {
    const [selected, setSelected] = useState([])

  useEffect(() => {
    const fetchFiltered = async () => {
      const query = selected.length === 0 ? ['ALL'] : selected;
      const res = await filterVulnerabilities(query);
      onFilter(res);
      console.log("Filtered vulnerabilities:", res);
    };

  fetchFiltered();
  }, [selected, onFilter]);

  const toggle = (value) => {
    setSelected((prev) =>
      prev.includes(value) ? prev.filter((v) => v !== value) : [...prev, value]
    )
  }

  return (
    <div className="severity-filter w-full text-left m-1" >
      <ul>
        {severities.map((sev) => (
          <li key={sev} className="flex items-center mb-1">
            <label className="">
              <input
                type="checkbox"
                checked={selected.includes(sev)}
                onChange={() => toggle(sev)}
                className="mr-1"
              />
              {sev}
            </label>
          </li>
        ))}
      </ul>
    </div>
  )
}
