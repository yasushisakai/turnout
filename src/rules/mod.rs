mod borda;
mod liquid;
mod majority;
mod approval;

pub use borda::Borda;
pub use liquid::LiquidDemocracy;
pub use majority::Majority;
pub use approval::Approval;

use crate::{address::Address, vote_data::Votes};
use nohash::IntMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoteOutcome {
    SimpleOutcome(IntMap<Address, f64>),
    OutcomeAndInfluences(IntMap<Address, f64>, IntMap<Address, f64>),
}

impl VoteOutcome {}

pub trait Rule {
    fn calculate(data: &Votes) -> VoteOutcome;
}
