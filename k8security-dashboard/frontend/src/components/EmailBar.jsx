import { useEffect, useState } from 'react'
import { addReceiverEmail, getReceiverEmails, setEmailStatus } from '../lib/api'
import "../styles/styles.css"


export default function EmailBar() {
    const [emails, setEmails] = useState([])
    const [newEmail, setNewEmail] = useState('')
    
    // Änderung 2
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
            <button onClick={async () => {
                        await addReceiverEmail(newEmail);
                        const updatedEmails = await getReceiverEmails();
                        setEmails(updatedEmails);
                        setNewEmail('');
            }}>
            </button>
            <div>
            {emails.map((email) => ( 
                <li key={email.id}>
                    <input
                        type="checkbox"
                        checked={email.receiving}
                        onChange={() => setEmailStatus(email.id)}
                    />
                    {email.email_adress}
                </li>
            ))}
            </div>
        </div>
    )
}