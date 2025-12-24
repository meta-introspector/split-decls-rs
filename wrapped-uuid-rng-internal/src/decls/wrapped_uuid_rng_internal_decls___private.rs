use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "getrandom")]
    pub use getrandom;
    #[cfg(feature = "rand")]
    pub use rand;
}
