import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { backend } from "../utils/backend";
import Navbar from "../components/Navbar";

export default function Test() {
  const { examId } = useParams();
  const [exam, setExam] = useState(null);
  const [currentQuestion, setCurrentQuestion] = useState(0);
  const [answers, setAnswers] = useState([]);

  useEffect(() => {
    backend.get_exam(Number(examId)).then(exam => {
      setExam(exam);
      setAnswers(new Array(exam.questions.length).fill(""));
    }).catch(console.error);
  }, [examId]);

  if (!exam) return <div style={{ padding: 48, textAlign: 'center' }}>Loading...</div>;

  const q = exam.questions[currentQuestion];

  function handleAnswer(val) {
    const newAnswers = [...answers];
    newAnswers[currentQuestion] = val;
    setAnswers(newAnswers);
  }

  async function handleSubmit() {
    try {
      const score = await backend.submit_answers(Number(examId), answers);
      alert("Your score: " + score);
    } catch (err) {
      alert("Error submitting answers: " + err);
    }
  }

  return (
    <div style={{ background: '#f7f9fc', minHeight: '100vh', width: '100vw', paddingTop: 48 }}>
      <Navbar />
      <div style={{ maxWidth: 700, margin: '0 auto', background: '#fff', borderRadius: 16, padding: 40, boxShadow: '0 2px 12px rgba(0,0,0,0.06)' }}>
        <h2 style={{ fontSize: '1.6rem', fontWeight: 700, marginBottom: 16 }}>Question {currentQuestion+1}</h2>
        <div style={{ color: '#7a8ca5', marginBottom: 24 }}>Question {currentQuestion+1} of {exam.questions.length}</div>
        <form>
          <div style={{ marginBottom: 32 }}>
            {q.answers.map((opt, i) => (
              <div key={i} style={{ display: 'flex', alignItems: 'center', marginBottom: 16 }}>
                <input type="radio" name="answer" id={`option${i}`} value={opt} checked={answers[currentQuestion]===opt} onChange={() => handleAnswer(opt)} style={{ marginRight: 12 }} />
                <label htmlFor={`option${i}`} style={{ fontSize: '1.1rem', color: '#444' }}>{opt}</label>
              </div>
            ))}
          </div>
          <div style={{ display: 'flex', gap: 16 }}>
            <button type="button" style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, fontSize: '1rem' }} disabled={currentQuestion===0} onClick={() => setCurrentQuestion(q => q-1)}>Previous</button>
            <button type="button" style={{ background: '#4a90e2', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, fontSize: '1rem' }} disabled={currentQuestion===exam.questions.length-1} onClick={() => setCurrentQuestion(q => q+1)}>Next</button>
            <button type="button" style={{ background: '#5bbf5b', color: '#fff', border: 'none', borderRadius: 8, padding: '10px 24px', fontWeight: 600, fontSize: '1rem' }} onClick={handleSubmit}>Submit Answers</button>
          </div>
        </form>
      </div>
    </div>
  );
}
