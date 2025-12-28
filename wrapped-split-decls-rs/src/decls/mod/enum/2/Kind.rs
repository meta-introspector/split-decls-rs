use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug)] enum Kind < Node : Idx > { # [doc = " A representation optimized for a small path graphs."] Path , General (Inner < Node >) , }