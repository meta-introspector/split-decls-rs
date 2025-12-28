use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct StringWrapper < 'a > (& 'a str) ;