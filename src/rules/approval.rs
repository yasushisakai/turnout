use nohash::IntMap;

use crate::vote_data::Votes;

use super::{Rule, VoteOutcome};

const APPROVAL_THRESHOLD: f64 = 0.5;

pub struct Approval {}

impl Rule for Approval {
    fn calculate(data: &Votes) -> VoteOutcome {
        let mut result = IntMap::default();
        for delegate in data.delegates.iter() {
            let Some(votes) = data.source_target_map.get(delegate) else {
                continue;
            };

            // assumes 'votes' are normalized
            for (target, value) in votes {
                if *value > APPROVAL_THRESHOLD {
                    result
                        .entry(*target)
                        .and_modify(|v| *v += 1.0)
                        .or_insert(1.0f64);
                }
            }
        }
        VoteOutcome::SimpleOutcome(result)
    }
}
