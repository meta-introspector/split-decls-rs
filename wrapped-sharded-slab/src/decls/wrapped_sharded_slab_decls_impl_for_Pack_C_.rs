use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<C: cfg::Config> Pack<C> for () {
    const BITS: usize = 0;
    const LEN: usize = 0;
    const SHIFT: usize = 0;
    const MASK: usize = 0;
    type Prev = ();
    fn as_usize(&self) -> usize {
        unreachable!()
    }
    fn from_usize(_val: usize) -> Self {
        unreachable!()
    }
    fn pack(&self, _to: usize) -> usize {
        unreachable!()
    }
    fn from_packed(_from: usize) -> Self {
        unreachable!()
    }
}
