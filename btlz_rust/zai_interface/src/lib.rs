//zai_interface/src/lib.rs
solana_program::declare_id!("HWjAY4TNEiAQquRKmwRXMabXf1PMGp36QyQgA162XdNr");
pub mod accounts;
pub use accounts::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
