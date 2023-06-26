use ndarray::{s, Array, Axis};
use nohash::IntMap;
use crate::{address::Address, vote_data::Votes};
use super::{
    Rule, VoteOutcome,
};

const ITERATION: u32 = 100;

#[derive(Debug)]
pub struct LiquidDemocracy {}

impl Rule for LiquidDemocracy {
    fn calculate(data: &Votes) -> VoteOutcome {
        // we want to  normalize the values first
        let matrix = data.create_matrix();

        println!("{:?}", matrix);

        let edge = matrix.shape()[0];
        let mut a = Array::eye(edge);
        let mut sum = Array::eye(edge);

        // TODO: can we use rayon
        for _ in 0..ITERATION {
            a = a.dot(&matrix);
            sum += &a;
        }

        let mut source_nodes: Vec<&Address> = data.delegates.iter().collect();
        let mut intermediaries: Vec<&Address> = data.intermediaries.iter().collect();
        source_nodes.append(&mut intermediaries);

        let a = a.slice(s![.., 0..data.delegates.len()]);
        let results = a.sum_axis(Axis(1)).slice(s![source_nodes.len()..]).to_vec();

        let poll_result: IntMap<Address, f64> =

            data.policies.iter().cloned().zip(results).collect();

        let sum = sum.slice(s![..source_nodes.len(), ..source_nodes.len()]);
        let sum_row = sum.sum_axis(Axis(1));
        let influence = (sum_row / sum.diag()).to_vec();

        let influence: IntMap<Address, f64> = source_nodes
            .iter()
            .map(|n| n.clone())
            .cloned()
            .zip(influence)
            .collect();

        VoteOutcome::OutcomeAndInfluences(poll_result, influence)
    }

}
