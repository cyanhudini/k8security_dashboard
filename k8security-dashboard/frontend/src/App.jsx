import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import './App.css'
import Dashboard from './pages/Dashboard'
import GroupedByPkgDashboard from './pages/GroupedByPkgDashboard'

function App() {


  return (
   <Router>
      <nav className="p-4 space-x-4">
        <Link to="/">Dashboard</Link>
        <Link to="/grouped">Grouped by Package</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/grouped" element={<GroupedByPkgDashboard />} />
      </Routes>
    </Router>
  )
}

export default App
