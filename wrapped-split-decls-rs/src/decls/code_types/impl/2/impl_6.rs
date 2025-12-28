use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Ord for CodeBlock { fn cmp (& self , other : & Self) -> Ordering { self . start_byte . cmp (& other . start_byte) . then_with (| | self . end_byte . cmp (& other . end_byte)) . then_with (| | self . start_line . cmp (& other . start_line)) . then_with (| | self . end_line . cmp (& other . end_line)) } }