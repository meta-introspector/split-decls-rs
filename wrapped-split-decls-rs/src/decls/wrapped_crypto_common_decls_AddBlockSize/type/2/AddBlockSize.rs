use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Alias for `AddBlockSize<A, B> = Sum<T, B::BlockSize>`"] pub type AddBlockSize < T , B > = Sum < T , < B as BlockSizeUser > :: BlockSize > ;
}