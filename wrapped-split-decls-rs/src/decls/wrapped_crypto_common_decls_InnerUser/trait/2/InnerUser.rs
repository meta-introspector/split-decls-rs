use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Types which use another type for initialization."] # [doc = ""] # [doc = " Generally it's used indirectly via [`InnerInit`] or [`InnerIvInit`]."] pub trait InnerUser { # [doc = " Inner type."] type Inner ; }