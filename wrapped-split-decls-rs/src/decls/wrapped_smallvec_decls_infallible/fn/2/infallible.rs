use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn infallible < T > (result : Result < T , CollectionAllocErr >) -> T { match result { Ok (x) => x , Err (CollectionAllocErr :: CapacityOverflow) => panic ! ("capacity overflow") , Err (CollectionAllocErr :: AllocErr { layout }) => alloc :: alloc :: handle_alloc_error (layout) , } }
}