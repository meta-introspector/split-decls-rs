use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ColorChoice { # [doc = " Get the current [`ColorChoice`] state"] pub fn global () -> Self { USER . get () } # [doc = " Override the detected [`ColorChoice`]"] pub fn write_global (self) { USER . set (self) ; } }