use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Terminator < 'a > (& 'a Arc < Registry >) ;