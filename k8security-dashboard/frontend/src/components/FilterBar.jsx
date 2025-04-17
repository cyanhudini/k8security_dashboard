import { useState } from 'react'


export default function FilterSidebar() {
    const [checked, setChecked] = useState(false);
    
    const handleCheckboxChange = (e) => {
        const check = !checked;
        setChecked(check);
        console.log(e.target.id)

    }

    return (
            <div>
                <h2 >Filter</h2>
                <div className="mb-4">
                    <label htmlFor="severity">Severity</label>
                    <div>
                        <input type="checkbox" id="CRITICAL" onChange={handleCheckboxChange} />
                        <label htmlFor="critical">Critical</label>

                    </div>
                </div>
            </div>
    )
}