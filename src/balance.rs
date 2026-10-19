//! Typed storage accessor layer and state management.
//!
//! Split into focused submodules by responsibility (Issue #647, #648):
//! - `positions`: user positions, share balances, deposits, lockups, and unbonding
//! - `shares`: share/amount conversion math
//! - `rewards`: reward rates, rate history, pool balances, checkpoints, and precision
//! - `governance_store`: proposals, voting weights, delegates, vetoes, and pending actions
//! - `burns`: fees burned, fee reserves, penalties, receipts, and slashing treasury
//! - `config`: protocol configuration, staker registries, changelog, and modular features

pub mod burns;
pub mod config;
pub mod governance_store;
pub mod positions;
pub mod rewards;
pub mod shares;

pub use burns::*;
pub use config::*;
pub use governance_store::*;
pub use positions::*;
pub use rewards::*;
pub use shares::*;
