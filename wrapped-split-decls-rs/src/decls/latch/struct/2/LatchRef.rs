use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " `&L` without any implication of `dereferenceable` for `Latch::set`"] pub (super) struct LatchRef < 'a , L > { inner : * const L , marker : PhantomData < & 'a L > , }