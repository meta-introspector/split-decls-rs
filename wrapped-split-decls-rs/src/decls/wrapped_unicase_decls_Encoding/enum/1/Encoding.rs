use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , Debug)] enum Encoding < S > { Ascii (Ascii < S >) , Unicode (Unicode < S >) , }