#![deny(clippy::all)]
#![deny(unsafe_code)]

pub mod core;
pub mod handler;
pub mod package;
mod server;

pub use server::*;
