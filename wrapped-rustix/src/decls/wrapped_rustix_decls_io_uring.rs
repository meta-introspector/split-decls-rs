use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(linux_kernel)]
#[cfg(feature = "io_uring")]
#[cfg_attr(docsrs, doc(cfg(feature = "io_uring")))]
pub mod io_uring;
