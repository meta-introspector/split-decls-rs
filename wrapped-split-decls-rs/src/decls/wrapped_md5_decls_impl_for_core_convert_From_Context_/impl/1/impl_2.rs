use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl core :: convert :: From < Context > for Digest { # [inline] fn from (context : Context) -> Digest { context . finalize () } }