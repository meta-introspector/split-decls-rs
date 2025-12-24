use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(
    any(
        feature = "fs",
        feature = "event",
        feature = "process",
        feature = "runtime",
        feature = "thread",
        feature = "time",
        all(feature = "event", any(bsd, linux_kernel, windows, target_os = "wasi")),
        all(
            linux_raw,
            not(feature = "use-libc-auxv"),
            not(feature = "use-explicitly-provided-auxv"),
            any(
                feature = "param",
                feature = "process",
                feature = "runtime",
                feature = "time",
                target_arch = "x86",
            )
        )
    )
)]
mod timespec;
