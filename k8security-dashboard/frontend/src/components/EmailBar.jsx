import { useEffect, useState } from 'react';
import { addReceiverEmail, getReceiverEmails, setEmailStatus } from '../lib/api';

export default function EmailBar() {
    const [emails, setEmails] = useState([]);
    const [newEmail, setNewEmail] = useState('');

    useEffect(() => {
        getReceiverEmails().then(setEmails);
    }, []);

    const handleCheckboxChange = async (emailId) => {
        await setEmailStatus(emailId);
        const updatedEmails = await getReceiverEmails();
        setEmails(updatedEmails);
    };

    return (
        <div className="emailbar w-full text-left m-1">
            <input
                type="email"
                id="email-input"
                value={newEmail}
                onChange={(e) => setNewEmail(e.target.value)}
                className="m-1 bg-cyan-900 rounded-md"
            />
            <button
                className="bg-gray-400"
                onClick={async () => {
                    await addReceiverEmail(newEmail);
                    const updatedEmails = await getReceiverEmails();
                    setEmails(updatedEmails);
                    setNewEmail('');
                }}
            >
                Add Email
            </button>
            <div className="email-list mt-4">
                <ul>
                    {emails
                        .sort((a, b) => a.email_adress.localeCompare(b.email_adress))
                        .map((email) => (
                            <li key={email.id}>
                                <input
                                    type="checkbox"
                                    checked={email.receiving}
                                    onChange={() => handleCheckboxChange(email.id)}
                                />
                                {email.email_adress}
                            </li>
                        ))}
                </ul>
            </div>
        </div>
    );
}
