import { useState, useEffect } from 'react'
import { filterVulnerabilities } from '../lib/api'

const severities = ['LOW', 'MEDIUM', 'HIGH', 'CRITICAL']

export default function FilterBar({ onFilter }) {
    const [selected, setSelected] = useState([])

    useEffect(() => {
        fetchFiltered(selected)
    }, [selected])

    const fetchFiltered = async (filters) => {
        const query = filters.length == 0 ? ['ALL'] : filters
        const res = await filterVulnerabilities(query)
        onFilter(res)
    }

  const toggle = (value) => {
    setSelected((prev) =>
      prev.includes(value) ? prev.filter((v) => v !== value) : [...prev, value]
    )
  }

  return (
    <div>
      {severities.map((sev) => (
        <label key={sev}>
          <input
            type="checkbox"
            checked={selected.includes(sev)}
            onChange={() => toggle(sev)}
          />
          {sev}
        </label>
      ))}
    </div>
  )
}
