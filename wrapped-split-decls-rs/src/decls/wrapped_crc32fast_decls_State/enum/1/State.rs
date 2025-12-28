use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone)] enum State { Baseline (baseline :: State) , Specialized (specialized :: State) , }
}