
import React, { useState } from "react";
import { backend } from "../utils/backend";
import { HttpAgent } from "@dfinity/agent";
import Navbar from "../components/Navbar";

export default function Dashboard() {
  const [user, setUser] = useState(null);
  const [certs, setCerts] = useState([]);
  const [principal, setPrincipal] = useState(null);
  React.useEffect(() => {
    const agent = new HttpAgent({ host: "http://127.0.0.1:4943" });
    agent.getPrincipal().then(setPrincipal);
  }, []);

  async function fetchUser() {
    try {
      if (!principal) throw new Error("Principal not loaded");
      const u = await backend.get_user(principal);
      setUser(u.Ok || null);
    } catch (err) {
      alert("Error fetching user: " + err);
    }
  }

  async function fetchCerts() {
    try {
      if (!principal) throw new Error("Principal not loaded");
      const c = await backend.get_certificates(principal);
      setCerts(c);
    } catch (err) {
      alert("Error fetching certificates: " + err);
    }
  }

  return (
    <div style={{ background: '#f7f9fc', minHeight: '100vh', width: '100vw', paddingTop: 48 }}>
      <Navbar />
      <div style={{ maxWidth: 900, margin: '0 auto', background: '#fff', borderRadius: 16, padding: 40, boxShadow: '0 2px 12px rgba(0,0,0,0.06)' }}>
        <h1 style={{ fontSize: '2rem', fontWeight: 700, marginBottom: 32 }}>Dashboard</h1>
        <div style={{ display: 'flex', gap: 32 }}>
          <div style={{ flex: 1, background: '#eaf0fa', borderRadius: 16, padding: 24, marginBottom: 24 }}>
            <div style={{ display: 'flex', alignItems: 'center', marginBottom: 24 }}>
              <div style={{ width: 56, height: 56, borderRadius: '50%', background: '#dbe6f7', marginRight: 16 }}></div>
              <div>
                <div style={{ fontWeight: 600, fontSize: '1.1rem' }}>{user ? user.principal.toText() : "User Name"}</div>
                <div style={{ color: '#7a8ca5', fontSize: '0.95rem' }}>{user ? user.role : "user@academyblock.com"}</div>
              </div>
            </div>
            <button style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, fontSize: '1rem', marginBottom: 16 }} onClick={fetchUser}>Get My User</button>
            <div style={{ fontWeight: 600, marginBottom: 8 }}>Completed Tests</div>
            <div style={{ background: '#fff', borderRadius: 8, padding: 16, minHeight: 48 }}>
              {user && user.exams_taken.length > 0 ? user.exams_taken.map(e => <div key={e} style={{ color: '#7a8ca5', fontSize: '1rem', marginBottom: 4 }}>Exam ID: {e}</div>) : <div style={{ color: '#7a8ca5' }}>No completed tests</div>}
            </div>
          </div>
          <div style={{ flex: 1, background: '#eaf0fa', borderRadius: 16, padding: 24, marginBottom: 24 }}>
            <div style={{ fontWeight: 600, marginBottom: 8 }}>Certificates</div>
            <button style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, fontSize: '1rem', marginBottom: 16 }} onClick={fetchCerts}>Get My Certificates</button>
            <div style={{ background: '#fff', borderRadius: 8, padding: 16, minHeight: 48 }}>
              {certs.length > 0 ? certs.map(c => (
                <div key={c.id} style={{ color: '#7a8ca5', fontSize: '1rem', marginBottom: 8 }}>
                  Certificate ID: {c.id}<br />Exam ID: {c.exam_id}<br />Score: {c.score}
                  <button style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '6px 18px', fontWeight: 600, fontSize: '0.95rem', marginTop: 8 }}>Resume</button>
                </div>
              )) : <div style={{ color: '#7a8ca5' }}>No certificates</div>}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
