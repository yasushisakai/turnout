use std::collections::BTreeMap;

use nohash::IntMap;
use ordered_float::OrderedFloat as Of;

use crate::{address::Address, vote_data::Votes};

use super::{
    Rule, VoteOutcome,
};

pub struct Borda {}

impl Rule for Borda {
    fn calculate(data: &Votes) -> VoteOutcome {
        let data = data.clone();
        let mut results = IntMap::default();
        for (_delegate, targets) in data.source_target_map {
            let mut pt = targets.len();
            let mut trans: BTreeMap<Of<f64>, Vec<Address>> = BTreeMap::new();
            for (target, value) in targets {
                trans
                    .entry(Of(value * -1.0f64)) // reverse order to big -> small
                    .and_modify(|v| v.push(target))
                    .or_insert(vec![target]);
            }

            for (_, list) in trans {
                let range_min = pt - list.len() + 1;
                let range_max = pt + 1;
                let points = (range_min..range_max).sum::<usize>() as f64 / list.len() as f64;
                for item in list.iter() {
                    results
                        .entry(*item)
                        .and_modify(|v| *v += points)
                        .or_insert(points);
                }
                pt -= list.len();
            }
        }

        VoteOutcome::SimpleOutcome(results)

    }

}
