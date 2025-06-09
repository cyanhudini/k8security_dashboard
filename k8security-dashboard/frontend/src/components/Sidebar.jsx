import FilterBar from './SeverityFilter';
import EmailBar from './EmailBar';
import React from 'react';
import {deleteVulnerabilities} from '../lib/api';


export default function Sidebar({onFilter, selected}) {
    


    const handleDelete = () => {
        // Logic to delete selected items
        console.log("Delete selected items:", selected);
        deleteVulnerabilities(selected);
    };
    return (
        <div className="sidebar">
            <h2 className="text-lg font-semibold mb-4">Filters</h2>
            <div className="actions-bar flex justify-end p-4">
                <button
                    className="bg-red-500 text-white px-4 py-2 rounded disabled:opacity-50"
                    onClick={handleDelete}
                    disabled={Object.keys(selected).length === 0}
                >
                    Delete Selected
                </button>
            </div>
            <FilterBar onFilter={onFilter} />
            <h2 className="text-lg font-semibold mt-6 mb-4">Notifications</h2>
            <EmailBar />
            
        </div>
    );
}