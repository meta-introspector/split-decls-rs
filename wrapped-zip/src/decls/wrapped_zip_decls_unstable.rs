use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/**Unstable APIs
All APIs accessible by importing this module are unstable; They may be changed in patch releases. You MUST use an exact version specifier in `Cargo.toml`, to indicate the version of this API you're using:
```toml

[dependencies]

zip = "=*/
#[doc = env!("CARGO_PKG_VERSION")]
/**"
```*/
pub mod unstable;
