use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline (always)] pub const fn is_ci () -> bool { option_env ! ("CI") . is_some () }
}