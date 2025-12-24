use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Easier access to common traits
pub mod prelude {
    pub use crate::IntoData;
    #[cfg(feature = "json")]
    pub use crate::IntoJson;
    pub use crate::ToDebug;
}
