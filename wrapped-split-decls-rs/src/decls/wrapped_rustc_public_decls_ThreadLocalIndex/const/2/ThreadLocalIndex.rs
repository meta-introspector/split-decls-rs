use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [expect (non_upper_case_globals)] # [doc = " Emulating unit struct `struct ThreadLocalIndex`;"] pub const ThreadLocalIndex : ThreadLocalIndex = ThreadLocalIndex { _phantom : PhantomData , } ;
}