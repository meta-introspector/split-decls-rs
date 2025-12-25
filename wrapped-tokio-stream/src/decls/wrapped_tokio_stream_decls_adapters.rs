use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Adapters for [`Stream`]s created by methods in [`StreamExt`].
pub mod adapters {
    pub use crate::stream_ext::{
        Chain, Filter, FilterMap, Fuse, Map, MapWhile, Merge, Peekable, Skip, SkipWhile, Take,
        TakeWhile, Then,
    };
    cfg_time! {
        pub use crate ::stream_ext:: { ChunksTimeout, Timeout, TimeoutRepeating };
    }
}
