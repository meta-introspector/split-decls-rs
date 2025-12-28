use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Structs that can produce gnuplot code"] trait Script { # [doc = " Translates some configuration struct into gnuplot code"] fn script (& self) -> String ; }