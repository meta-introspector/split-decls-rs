use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () , Box < dyn std :: error :: Error > > { bootstrap3 :: test_bootstrap3 () ? ; Ok (()) }