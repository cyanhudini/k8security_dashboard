import { useEffect, useState } from 'react'
import { addReceiverEmail, getReceiverEmails } from '../lib/api'
import "../styles/styles.css"


export default function EmailBar() {
    const [emails, setEmails] = useState([])
    const [newEmail, setNewEmail] = useState('')
    useEffect(() => {
        getReceiverEmails().then(setEmails)
    }, [])

    return (
        <div className="emailbar">
            <label for="email"></label>
            <input
                type="email"
                id="email"
                value={newEmail}
                onChange={e => setNewEmail(e.target.value)}
            />
            <button onClick={() => addReceiverEmail(newEmail)}></button>
            <div>
            {emails.map((email) => ( 
                <li key={email}>
                    <input
                        type="checkbox"
                        checked
                        
                    />
                    {email.email_adress}
                </li>
            ))}
            </div>
        </div>
    )
}