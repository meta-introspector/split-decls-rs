use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Construct a new `GlobWalker` with a glob pattern."] # [doc = ""] # [doc = " When iterated, the current directory will be recursively searched for paths"] # [doc = " matching `pattern`, unless the pattern specifies an absolute path."] pub fn glob < S : AsRef < str > > (pattern : S) -> Result < GlobWalker , GlobError > { glob_builder (pattern) . build () }
}