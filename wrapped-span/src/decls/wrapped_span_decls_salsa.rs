use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "salsa"))]
mod salsa {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Id(u32);
}
