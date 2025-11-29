// src/lib.rs
pub mod clients;
pub mod storage;
pub mod error;
pub mod models;
pub mod config;

// Re-export commonly used items for easier access
pub use error::AppError;
