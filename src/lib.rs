//! # Sundaram Sieve 64-bit Library
//!
//! A Rust library for finding prime numbers within a given interval.
//! Generalized Sundaram Sieve for 64-bit integers.
//! Both sequential and parallel execution modes are provided.

mod bitset;
pub mod sieve_par;
pub mod sieve_seq;

// Re-export only the public API functions
pub use sieve_par::sundaram_sieve_par;
pub use sieve_seq::sundaram_sieve_seq;
