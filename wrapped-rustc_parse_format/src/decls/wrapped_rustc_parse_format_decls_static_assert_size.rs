use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(test, target_pointer_width = "64"))]
rustc_index::static_assert_size!(Piece<'_>, 16);
