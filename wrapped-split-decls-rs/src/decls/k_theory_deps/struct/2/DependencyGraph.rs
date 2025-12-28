use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct DependencyGraph { forward : HashMap < usize , Vec < usize > > , reverse : HashMap < usize , Vec < usize > > , node_count : usize , }