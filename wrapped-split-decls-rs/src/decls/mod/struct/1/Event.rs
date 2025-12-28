use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct Event < N > { node : N , becomes : NodeStatus , }