import React from "react";
import { Link, useLocation } from "react-router-dom";

export default function Navbar() {
  const location = useLocation();
  const navItems = [
    { to: "/", label: "Home" },
    { to: "/tests", label: "Tests" },
    { to: "/dashboard", label: "Dashboard" },
    { to: "#", label: "Profile" },
  ];
  return (
    <nav style={{ background: '#4a90e2', padding: '16px 32px', borderRadius: '0 0 16px 16px', display: 'flex', justifyContent: 'flex-end', gap: 32 }}>
      {navItems.map(item => (
        <Link
          key={item.label}
          to={item.to}
          style={{
            color: '#fff',
            fontWeight: 500,
            marginRight: item.label !== 'Profile' ? 24 : 0,
            textDecoration: location.pathname === item.to ? 'underline' : 'none',
            fontSize: '1.1rem',
          }}
        >
          {item.label}
        </Link>
      ))}
    </nav>
  );
}