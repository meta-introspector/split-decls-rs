use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Whether or not this running in a Continuous Integration environment."] fn is_ci () -> bool { option_env ! ("CI") . is_some () || option_env ! ("TF_BUILD") . is_some () }
}