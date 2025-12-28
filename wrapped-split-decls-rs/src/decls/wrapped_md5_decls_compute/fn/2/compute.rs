use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Compute the digest of data."] # [inline] pub fn compute < T : AsRef < [u8] > > (data : T) -> Digest { let mut context = Context :: new () ; context . consume (data) ; context . finalize () }