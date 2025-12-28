use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for CodeBlock { fn eq (& self , other : & Self) -> bool { self . start_byte == other . start_byte && self . end_byte == other . end_byte && self . start_line == other . start_line && self . end_line == other . end_line && self . fingerprint == other . fingerprint } }