use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, target_os = "espidf", target_os = "wasi")))]
#[cfg(
    any(
        feature = "io_uring",
        feature = "process",
        feature = "runtime",
        all(bsd, feature = "event")
    )
)]
mod signal;
