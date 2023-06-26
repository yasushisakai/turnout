use ndarray::{concatenate, Array, Array2, Axis};
use nohash::{IntMap, IntSet};
use serde::{Deserialize, Serialize};

use crate::address::Address;
pub type SourceTargetMap = IntMap<Address, IntMap<Address, f64>>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Votes {
    pub delegates: IntSet<Address>,
    pub intermediaries: IntSet<Address>,
    pub policies: IntSet<Address>,
    pub source_target_map: SourceTargetMap,
}

impl Votes {
    #[allow(dead_code)]
    pub fn count_address(&self) -> usize {
        self.delegates.len() + self.intermediaries.len() + self.policies.len()
    }

    #[allow(dead_code)]
    fn white_list(&self) -> Vec<&Address> {
        let mut wl = Vec::new();
        let mut connected: Vec<&Address> = self.delegates.iter().collect();
        loop {
            let mut new_connected = Vec::new();
            for a in connected.iter() {
                let next = self.source_target_map.get(&a);
                if let Some(votes) = next {
                    for address in votes.keys() {
                        if let (None, None) = (
                            wl.iter().position(|wl_address| wl_address == &address),
                            new_connected
                                .iter()
                                .position(|nc_address| nc_address == &address),
                        ) {
                            new_connected.push(address);
                        }
                    }
                }
            }
            wl.extend_from_slice(&connected);
            if new_connected.is_empty() {
                break;
            }
            connected = new_connected;
        }
        wl
    }

    pub fn strip_addresses(&mut self) {
        let white_list: Vec<Address> = self.white_list().iter().map(|e| *e.clone()).collect();

        self.intermediaries = self
            .intermediaries
            .iter()
            .filter(|a| white_list.iter().any(|w| w == *a))
            .map(|a| a.clone())
            .collect();

        self.policies = self
            .policies
            .iter()
            .filter(|a| white_list.iter().any(|w| w == *a))
            .copied()
            .collect();
    }

    /// the source is restricted to delegates and the targets are only
    /// policies
    pub fn delegates_to_policies(&mut self) {
        let mut result = IntMap::default();

        for d in self.delegates.iter() {
            let Some(votes) = self.source_target_map.get(d) else {
               result.insert(*d, IntMap::default());
               continue;
            };

            let votes: IntMap<Address, f64> = votes
                .iter()
                .filter(|(address, _)| self.policies.iter().any(|p| &p == address))
                .map(|(address, value)| (*address, *value))
                .collect();

            result.insert(*d, votes);
        }

        self.source_target_map = result;
    }

    pub fn delegates_to_delegates(&mut self) {
        let mut result = IntMap::default();

        for d in self.delegates.iter() {
            let Some(votes) = self.source_target_map.get(d) else {
               result.insert(*d, IntMap::default());
               continue;
            };

            let votes: IntMap<Address, f64> = votes
                .iter()
                .filter(|(address, _)| self.delegates.iter().any(|p| &p == address))
                .map(|(address, value)| (*address, *value))
                .collect();

            result.insert(*d, votes);
        }
        self.source_target_map = result
    }

    pub fn delegates_to_intermediaries(&mut self) {
        let mut result = IntMap::default();

        for d in self.delegates.iter() {
            let Some(votes) = self.source_target_map.get(d) else {
               result.insert(*d, IntMap::default());
               continue;
            };

            let votes: IntMap<Address, f64> = votes
                .iter()
                .filter(|(address, _)| self.intermediaries.iter().any(|p| &p == address))
                .map(|(address, value)| (*address, *value))
                .collect();

            result.insert(*d, votes);
        }
        self.source_target_map = result;
    }

    pub fn normalize(&mut self) {
        let mut map: IntMap<Address, IntMap<Address, f64>> = IntMap::default();
        for (source, targets) in self.source_target_map.iter() {
            let sum = targets.iter().fold(0.0, |acc, (_, v)| acc + v);
            let normalized: IntMap<Address, f64> =
                targets.into_iter().map(|(a, v)| (*a, *v / sum)).collect();
            map.insert(*source, normalized);
        }
        self.source_target_map = map;
    }

    pub fn create_matrix(&self) -> Array2<f64> {
        let num_delegates = self.delegates.len();
        let num_intermediaries = self.intermediaries.len();
        let num_policies = self.policies.len();

        let mut d_to_p: Array2<f64> = Array::zeros((
            num_delegates + num_intermediaries + num_policies,
            num_delegates + num_intermediaries,
        ));

        for (si, source) in self.delegates.iter().enumerate() {
            if let Some(votes) = self.source_target_map.get(source) {
                for (destination, value) in votes {
                    if let Some(i) = self.delegates.iter().position(|p| p == destination) {
                        d_to_p[[i, si]] = *value;
                        continue;
                    }
                    let mut di: usize = num_delegates;
                    if let Some(i) = self.intermediaries.iter().position(|p| p == destination) {
                        di += i;
                        d_to_p[[di, si]] = *value;
                        continue;
                    }
                    di += num_intermediaries;

                    di += self.policies.iter().position(|p| p == destination).unwrap();

                    d_to_p[[di, si]] = *value;
                }
            }
        }

        for (ii, source) in self.intermediaries.iter().enumerate() {
            let si = ii + num_delegates;
            if let Some(votes) = self.source_target_map.get(source) {
                for (destination, value) in votes {
                    if let Some(i) = self.delegates.iter().position(|p| p == destination) {
                        d_to_p[[i, si]] = *value;
                        continue;
                    }

                    let mut di: usize = num_delegates;
                    // the user votes for some intermediary
                    if let Some(i) = self.intermediaries.iter().position(|p| p == destination) {
                        di += i;
                        d_to_p[[di, si]] = *value;
                        continue;
                    }

                    di += num_intermediaries;
                    di += self.policies.iter().position(|p| p == destination).unwrap();

                    d_to_p[[di, si]] = *value;
                }
            }
        }

        let p_to_d: Array2<f64> = Array::zeros((num_policies, num_intermediaries + num_delegates));
        let p_to_p: Array2<f64> = Array::eye(self.policies.len());

        let left: Array2<f64> = concatenate![Axis(1), p_to_d, p_to_p];
        concatenate![Axis(1), d_to_p, left.t()]
    }
}
