use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Idx > From < DenseBitSet < T > > for GrowableBitSet < T > { fn from (bit_set : DenseBitSet < T >) -> Self { Self { bit_set } } }