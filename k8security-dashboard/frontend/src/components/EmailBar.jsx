import { useEffect, useState } from 'react'
import { getReceiverEmails } from '../lib/api'
import "../styles/styles.css"


export default function EmailBar() {
    const [emails, setEmails] = useState([])
    useEffect(() => {
        getReceiverEmails().then(setEmails)
    }, [emails])
    const toggleStatus = (value) => {
        
    }


    return (
        <div className="emailbar">
            <input
                type="email" />
            
            {emails.map((email) => ( 
                <label key={email}>
                    <input
                        type="checkbox"
                        checked
                        onChange={() => toggleStatus(email)}
                    />
                    {email.email_adress}
                </label>
            ))}
        </div>
    )
}