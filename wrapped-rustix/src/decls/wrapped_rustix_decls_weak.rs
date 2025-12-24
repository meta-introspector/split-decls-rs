use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    any(
        all(linux_raw, feature = "use-libc-auxv"),
        all(libc, not(any(windows, target_os = "espidf", target_os = "wasi")))
    )
)]
#[macro_use]
mod weak;
