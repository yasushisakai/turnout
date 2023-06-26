use nohash::IntMap;
use crate::{address::Address, vote_data::{Votes, SourceTargetMap}};

use super::{
    Rule, VoteOutcome,
};

pub struct Majority {}

impl Majority {
    pub fn single_favorite(data: &mut Votes) {
        let mut new_votes: SourceTargetMap = IntMap::default();
        for delegate in data.delegates.iter() {
            let Some(votes) = data.source_target_map.get(&delegate) else {
                continue;
            };
            let mut max = f64::MIN;
            let mut max_address = None;

            for (address, value) in votes {
                if &max < value {
                    max = *value;
                    max_address = Some(*address);
                }
            }
            if let Some(address) = max_address {
                new_votes
                    .entry(*delegate)
                    .and_modify(|t| {
                        t.insert(address, 1.0);
                    })
                    .or_insert_with(|| {
                        let mut targets = IntMap::default();
                        targets.insert(address, 1.0);
                        targets
                    });
            }
        }
        data.source_target_map = new_votes;
    }
}

impl Rule for Majority {
    fn calculate(data: &Votes) -> VoteOutcome {
        let mut favorites: IntMap<Address, f64> = IntMap::default();
        for delegate in data.delegates.iter() {
            let Some(votes) = data.source_target_map.get(&delegate) else {
                continue;
            };
            let mut max = f64::MIN;
            let mut max_address = None;

            for (address, value) in votes {
                if &max < value {
                    max = *value;
                    max_address = Some(address)
                }
            }
            if let Some(address) = max_address {
                favorites
                    .entry(*address)
                    .and_modify(|v| *v += 1_f64)
                    .or_insert(1_f64);
            }
        }
        VoteOutcome::SimpleOutcome(favorites)
    }
}
