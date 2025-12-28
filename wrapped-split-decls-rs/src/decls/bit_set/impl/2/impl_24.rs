use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Idx > From < GrowableBitSet < T > > for DenseBitSet < T > { fn from (bit_set : GrowableBitSet < T >) -> Self { bit_set . bit_set } }
}