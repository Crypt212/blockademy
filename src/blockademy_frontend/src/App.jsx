import React from "react";
import "./App.css";

const learningPaths = [
  {
    id: 1,
    name: "English Fluency Test",
    description: "Assess your grammar, vocabulary, and reading comprehension levels.",
    language: "English",
    languageColor: "#3b82f6",
    stars: 780,
    forks: 340,
  },
  {
    id: 2,
    name: "JavaScript Skill Path",
    description: "Test your JavaScript fundamentals and earn skill badges on-chain.",
    language: "JavaScript",
    languageColor: "#f1e05a",
    stars: 630,
    forks: 220,
  },
  {
    id: 3,
    name: "IQ Challenge Series",
    description: "Evaluate your logic, reasoning, and problem-solving under pressure.",
    language: "Logic",
    languageColor: "#7c3aed",
    stars: 540,
    forks: 180,
  },
  {
    id: 4,
    name: "Python Basics Test",
    description: "Validate your Python syntax and algorithmic thinking skills.",
    language: "Python",
    languageColor: "#3572A5",
    stars: 490,
    forks: 150,
  }
];

const StarIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
    <path d="M2.866 14.85c-.078.444.36.791.746.593l4.39-2.256 4.389 2.256c.386.198.824-.149.746-.592l-.83-4.73 3.522-3.356c.33-.314.16-.888-.282-.95l-4.898-.696L8.465.792a.513.513 0 0 0-.927 0L5.354 5.12l-4.898.696c-.441.062-.612.636-.283.95l3.523 3.356-.83 4.73zm4.905-2.767-3.686 1.894.694-3.957a.565.565 0 0 0-.163-.505L1.71 6.745l4.052-.576a.525.525 0 0 0 .393-.288L8 2.223l1.847 3.658a.525.525 0 0 0 .393.288l4.052.575-2.906 2.77a.565.565 0 0 0-.163.506l.694 3.957-3.686-1.894a.503.503 0 0 0-.461 0z"/>
  </svg>
);

const ForkIcon = () => (
  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
    <path fillRule="evenodd" d="M5 3.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0zm0 2.122a2.25 2.25 0 1 0-1.5 0v.878A2.25 2.25 0 0 0 5.75 8.5h1.5v2.128a2.251 2.251 0 1 0 1.5 0V8.5h1.5a2.25 2.25 0 0 0 2.25-2.25v-.878a2.25 2.25 0 1 0-1.5 0v.878a.75.75 0 0 1-.75.75h-4.5A.75.75 0 0 1 5 6.25v-.878zM10.5 12.75a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0zm-3.75 0a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0z"/>
  </svg>
);

const Header = () => (
  <nav className="navbar navbar-expand-lg navbar-dark fixed-top">
    <div className="container">
      <a className="navbar-brand fs-4" href="#">Blockademy</a>
      <button className="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
        <span className="navbar-toggler-icon"></span>
      </button>
      <div className="collapse navbar-collapse" id="navbarNav">
        <ul className="navbar-nav ms-auto">
          <li className="nav-item"><a className="nav-link" href="#">Tests</a></li>
          <li className="nav-item"><a className="nav-link" href="#">Tracks</a></li>
          <li className="nav-item"><a className="nav-link" href="#">My Wallet</a></li>
        </ul>
      </div>
    </div>
  </nav>
);

const HeroSection = () => (
  <section className="hero-section text-center text-white">
    <div className="container">
      <h1 className="display-4">Test Your Skills. Prove It On-Chain.</h1>
      <p className="lead col-lg-8 mx-auto">
        Blockademy is the all-in-one platform to assess your skills in coding, languages, IQ, and more — with every achievement stored on blockchain as verifiable proof.
      </p>
      <div className="d-grid gap-2 d-sm-flex justify-content-sm-center mt-4">
        <button type="button" className="btn btn-primary btn-lg">Start Testing</button>
        <button type="button" className="btn btn-outline-light btn-lg">Connect Wallet</button>
      </div>
    </div>
  </section>
);

const FeatureCard = ({ title, children }) => (
  <div className="col-lg-4">
    <div className="card h-100 p-4 text-white feature-card">
      <div className="card-body">
        <h3 className="card-title h4 mb-3">{title}</h3>
        <p className="card-text text-white-50">{children}</p>
      </div>
    </div>
  </div>
);

const FeaturesSection = () => (
  <section className="py-5">
    <div className="container">
      <h2 className="text-center text-white mb-5">Why Use Blockademy?</h2>
      <div className="row g-4">
        <FeatureCard title="On-Chain Proof">
          Every skill test you pass is recorded on blockchain. No central authority needed.
        </FeatureCard>
        <FeatureCard title="Skill Diversity">
          Measure your level in coding, languages, logical reasoning, and more — all in one place.
        </FeatureCard>
        <FeatureCard title="No Signups">
          Your Web3 wallet is your profile. No emails, no passwords — just progress.
        </FeatureCard>
      </div>
    </div>
  </section>
);

const PathCard = ({ repo }) => (
  <div className="col-lg-6">
    <div className="card h-100 p-3 repo-card">
      <div className="card-body d-flex flex-column">
        <div>
          <h5 className="card-title"><a href="#">{repo.name}</a></h5>
          <p className="card-text text-white-50 mb-3">{repo.description}</p>
        </div>
        <div className="repo-stats mt-auto">
          <span><span className="language-dot" style={{ backgroundColor: repo.languageColor }}></span> {repo.language}</span>
          <span className="ms-3"><StarIcon /> {repo.stars}</span>
          <span className="ms-3"><ForkIcon /> {repo.forks}</span>
        </div>
      </div>
    </div>
  </div>
);

const LearningPathsSection = ({ paths }) => (
  <section className="py-5 bg-dark">
    <div className="container">
      <div className="d-flex justify-content-between align-items-center mb-5 flex-wrap gap-3">
        <h2 className="text-white mb-0">Explore Skill Tracks</h2>
        <button type="button" className="btn btn-primary">See All</button>
      </div>
      <div className="row g-4">
        {paths.map(path => <PathCard key={path.id} repo={path} />)}
      </div>
    </div>
  </section>
);

const Footer = () => (
  <footer className="footer py-4 mt-auto">
    <div className="container text-center">
      <span className="text-white-50">&copy; 2025 Blockademy. All Rights Reserved.</span>
    </div>
  </footer>
);

function App() {
  return (
    <>
      <Header />
      <main>
        <HeroSection />
        <FeaturesSection />
        <LearningPathsSection paths={learningPaths} />
      </main>
      <Footer />
    </>
  );
}

export default App;