use serde::{Deserialize, Serialize};
use std::collections::HashMap;

enum ArgKind { None , Input , Output , Filter , Rustfmt , Reference , Derive , Link , }