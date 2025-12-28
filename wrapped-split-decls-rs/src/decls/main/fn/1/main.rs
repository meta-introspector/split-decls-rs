use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () , Box < dyn std :: error :: Error > > { bootstrap3 :: test_bootstrap3 () ? ; Ok (()) }
}