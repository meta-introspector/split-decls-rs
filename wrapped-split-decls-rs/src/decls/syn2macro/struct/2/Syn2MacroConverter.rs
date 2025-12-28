use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Main syn2macro converter with Bott periodicity awareness"] pub struct Syn2MacroConverter { security : SecurityContext , bott_generator : Option < BottMacroGenerator > , }