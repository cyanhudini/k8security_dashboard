import {  useState } from 'react'


export default function FilterBar() {
    const [severities, setSeverities] = useState([])


    return (
        <div>
            <label for="dropdown">filter by Severity</label>
            <select name="" id="severity">
                <option value="LOW">LOW</option>
            </select>
        </div>
    )
}