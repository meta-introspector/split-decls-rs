use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (not (feature = "std"))] # [doc (hidden)] pub type Once = self :: spin :: Once < () > ;