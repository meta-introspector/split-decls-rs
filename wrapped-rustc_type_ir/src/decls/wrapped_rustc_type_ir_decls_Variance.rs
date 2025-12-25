use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "nightly", derive(Decodable, Encodable, HashStable_NoContext))]
#[cfg_attr(feature = "nightly", rustc_pass_by_value)]
pub enum Variance {
    Covariant,
    Invariant,
    Contravariant,
    Bivariant,
}
