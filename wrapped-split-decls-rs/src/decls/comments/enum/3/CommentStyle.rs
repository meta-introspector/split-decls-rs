use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Debug)] pub enum CommentStyle { # [doc = " No code on either side of each line of the comment"] Isolated , # [doc = " Code exists to the left of the comment"] Trailing , # [doc = " Code before /* foo */ and after the comment"] Mixed , # [doc = " Just a manual blank line \"\\n\\n\", for layout"] BlankLine , }