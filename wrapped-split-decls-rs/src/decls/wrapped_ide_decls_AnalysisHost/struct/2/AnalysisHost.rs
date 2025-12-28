use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " `AnalysisHost` stores the current state of the world."] # [derive (Debug)] pub struct AnalysisHost { db : RootDatabase , }