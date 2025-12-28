use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [repr (C)] pub struct SmallVec < T , const N : usize > { len : TaggedLen , raw : RawSmallVec < T , N > , _marker : PhantomData < T > , }