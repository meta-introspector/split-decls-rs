use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Name of a diagnostic argument."] pub type DiagArgName = Cow < 'static , str > ;