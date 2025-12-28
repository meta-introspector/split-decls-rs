use serde::{Deserialize, Serialize};
use std::collections::HashMap;

enum HashStableMode { Normal , Generic , NoContext , }