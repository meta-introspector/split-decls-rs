use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    not(
        any(
            windows,
            target_os = "android",
            target_os = "espidf",
            target_os = "horizon",
            target_os = "vita",
            target_os = "wasi"
        )
    )
)]
#[cfg(feature = "shm")]
#[cfg_attr(docsrs, doc(cfg(feature = "shm")))]
pub mod shm;
