use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Nodes < 'a , N > = Cow < 'a , [N] > ;