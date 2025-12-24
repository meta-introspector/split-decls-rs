use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod prelude {
    #[cfg(feature = "accumulator")]
    pub use crate::accumulator::Accumulator;
    pub use crate::{Database, Setter};
}
