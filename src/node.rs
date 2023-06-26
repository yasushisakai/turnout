use nohash::IntMap;
use crate::address::Address;

pub trait Node {
    fn address(&self) -> Address;
    fn targets(&self) -> Option<IntMap<Address, f64>>;
}

