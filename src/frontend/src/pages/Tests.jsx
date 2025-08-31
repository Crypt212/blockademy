import React, { useEffect, useState } from "react";
import { backend } from "../utils/backend";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar";

export default function Tests() {
  const [exams, setExams] = useState([]);
  const [search, setSearch] = useState("");
  const [filter, setFilter] = useState("");

  useEffect(() => {
    backend.list_exams().then(setExams).catch(console.error);
  }, []);

  const filtered = exams.filter(exam =>
    (!search || exam.title.toLowerCase().includes(search.toLowerCase())) &&
    (!filter || exam.level === filter)
  );

  return (
    <div style={{ background: '#f7f9fc', minHeight: '100vh', width: '100vw', paddingTop: 48 }}>
      <Navbar />
      <div style={{ maxWidth: 900, margin: '0 auto', background: '#fff', borderRadius: 16, padding: 40, boxShadow: '0 2px 12px rgba(0,0,0,0.06)' }}>
        <h1 style={{ fontSize: '2rem', fontWeight: 700, marginBottom: 32 }}>Tests</h1>
        <div style={{ display: 'flex', gap: 16, marginBottom: 32 }}>
          <input type="text" style={{ flex: 1, padding: '12px 16px', borderRadius: 8, border: '1px solid #dbe6f7', fontSize: '1rem' }} placeholder="Search tests..." value={search} onChange={e => setSearch(e.target.value)} />
          <select style={{ padding: '12px 16px', borderRadius: 8, border: '1px solid #dbe6f7', fontSize: '1rem', minWidth: 120 }} value={filter} onChange={e => setFilter(e.target.value)}>
            <option value="">Filter</option>
            <option value="Beginner">Beginner</option>
            <option value="Intermediate">Intermediate</option>
            <option value="Advanced">Advanced</option>
          </select>
        </div>
        <div style={{ display: 'flex', gap: 24, flexWrap: 'wrap' }}>
          {filtered.map(exam => (
            <div key={exam.id} style={{ background: '#f7f9fc', borderRadius: 12, padding: 24, minWidth: 220, flex: '1 1 220px', boxShadow: '0 1px 6px rgba(0,0,0,0.04)', display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: 24 }}>
              <div style={{ height: 48, width: '100%', background: '#eaf0fa', borderRadius: 8, marginBottom: 16 }}></div>
              <h3 style={{ fontSize: '1.1rem', fontWeight: 600, marginBottom: 8 }}>{exam.title}</h3>
              <div style={{ color: '#7a8ca5', fontSize: '0.95rem', marginBottom: 16 }}>Level: {exam.level || "N/A"}</div>
              <Link to={`/test/${exam.id}`} style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, textDecoration: 'none', fontSize: '1rem' }}>Start Test</Link>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
