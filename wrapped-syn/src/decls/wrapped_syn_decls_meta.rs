use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "parsing", any(feature = "full", feature = "derive")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "parsing", any(feature = "full", feature = "derive"))))
)]
pub mod meta;
