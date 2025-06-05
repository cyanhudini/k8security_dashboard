import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import './App.css'
import Dashboard from './pages/Dashboard'
import GroupedByPkgDashboard from './pages/GroupedByPkgDashboard'
import  GroupedByScanType from './pages/GroupedByScanType'
function App() {


  return (
   <Router>
      <nav className="p-4 space-x-4">
        <Link to="/">Dashboard</Link>
        <Link to="/group_by_pkg">Grouped by Package</Link>
        <Link to="/group_vulns">Group By Scan Type</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/group_by_pkg" element={<GroupedByPkgDashboard />} />
        <Route path="/group_vulns" element={<GroupedByScanType />} />
      </Routes>
    </Router>
  )
}

export default App
