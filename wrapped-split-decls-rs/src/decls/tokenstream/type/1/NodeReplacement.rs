use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " See the comment on `ParserReplacement`."] pub type NodeReplacement = (NodeRange , Option < AttrsTarget >) ;