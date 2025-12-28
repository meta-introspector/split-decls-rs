use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " See the comment on `ParserReplacement`."] pub type NodeReplacement = (NodeRange , Option < AttrsTarget >) ;
}