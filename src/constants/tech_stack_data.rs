use crate::types::tech_stack_types::TechStack;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref TECH_STACK: Vec<TechStack> = vec![
    TechStack {
      key: "Solidity / Foundry / Hardhat".to_string(),
      description: "Smart contract development and testing".to_string(),
    },
    TechStack {
      key: "Wagmi / Ether.js / Alloy".to_string(),
      description: "Web3 frontend interact and observe data".to_string(),
    },
    TechStack {
      key: "Next.js / React".to_string(),
      description: "Best choice for frontend development".to_string(),
    },
    TechStack {
      key: "React Native".to_string(),
      description: "Only one of mobile development that I can do!".to_string(),
    },
    TechStack {
      key: "Node / Express / Nest.js".to_string(),
      description: "Backend Node universe for quick development".to_string(),
    },
    TechStack {
      key: "Tailwind CSS / SASS".to_string(),
      description: "Frontend styled for quick development".to_string(),
    },
    TechStack {
      key: "Day.js".to_string(),
      description: "Small size and simple to use for time management".to_string(),
    },
    TechStack {
      key: "React Hook Form".to_string(),
      description: "The best validate form because I used to apply only one!".to_string(),
    },
    TechStack {
      key: "Circom".to_string(),
      description: "First ZKP language that I learned!".to_string(),
    },
    TechStack {
      key: "Gin / Fiber / Echo".to_string(),
      description: "Golang backend universe for large scaling".to_string(),
    },
    TechStack {
      key: "Axum".to_string(),
      description: "First Rust backend that I learned!".to_string(),
    },
    TechStack {
      key: "Polars (Rust) / Pandas".to_string(),
      description: "Migrated all projects from pandas to polars".to_string(),
    },
    TechStack {
      key: "Flask".to_string(),
      description: "Once upon a time, I used to clone Etherscan".to_string(),
    },
    TechStack {
      key: "MySQL / SQLite / MongoDB".to_string(),
      description: "Simple database use around the world".to_string(),
    },
    TechStack {
      key: "Google Cloud".to_string(),
      description: "The place for real project deployment".to_string(),
    },
    TechStack {
      key: "Railway / Vercel".to_string(),
      description: "POC of some services and testing frontend deployment".to_string(),
    },
  ];
}
