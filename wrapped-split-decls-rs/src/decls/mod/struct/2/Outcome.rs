use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct Outcome < O , E > { # [doc = " Backtrace of obligations that were found to be in error."] pub errors : Vec < Error < O , E > > , }