use nohash::IntMap;
use sha2::{Sha256, Digest};

use crate::{node::Node, address::Address};

pub struct SimpleNode(String);

impl Node for SimpleNode {
    fn address(&self) -> Address {
        let mut hasher = Sha256::new();
        hasher.update(self.0.as_bytes());
        let digest: [u8;32] = hasher.finalize().into();
        digest.try_into().expect("should not fail.")
    }

    fn targets(&self) -> Option<IntMap<Address, f64>> {
        None
    }
}
