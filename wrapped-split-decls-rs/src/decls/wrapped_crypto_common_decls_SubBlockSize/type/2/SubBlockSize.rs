use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Alias for `SubBlockSize<A, B> = Diff<T, B::BlockSize>`"] pub type SubBlockSize < T , B > = Diff < T , < B as BlockSizeUser > :: BlockSize > ;