import React from "react";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar";

export default function Home() {
  return (
    <div style={{ background: '#eaf0fa', minHeight: '100vh', width: '100vw' }}>
      <Navbar />
      <div style={{ maxWidth: 600, margin: '48px auto', background: '#4a90e2', borderRadius: 24, padding: 40, color: '#fff', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div>
          <h1 style={{ fontSize: '2.8rem', fontWeight: 700, marginBottom: 16 }}>AcademyBlock</h1>
          <p style={{ fontSize: '1.25rem', marginBottom: 32 }}>Learn, Test, Certify on the Blockchain</p>
          <Link to="/tests" style={{ background: '#f97c5b', color: '#fff', border: 'none', borderRadius: 8, padding: '12px 32px', fontSize: '1.1rem', fontWeight: 600, textDecoration: 'none' }}>Get Started</Link>
        </div>
        <div>
          <img src="https://cdn-icons-png.flaticon.com/512/833/833524.png" alt="AcademyBlock" style={{ width: 100, background: '#eaf0fa', borderRadius: 16, padding: 16 }} />
        </div>
      </div>
    </div>
  );
}
