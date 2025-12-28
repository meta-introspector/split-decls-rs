use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] enum State { Baseline (baseline :: State) , Specialized (specialized :: State) , }