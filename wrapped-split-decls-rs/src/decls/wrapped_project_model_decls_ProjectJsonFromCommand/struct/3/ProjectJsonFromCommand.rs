use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , PartialEq , Eq)] pub struct ProjectJsonFromCommand { # [doc = " The data describing this project, such as its dependencies."] pub data : ProjectJsonData , # [doc = " The build system specific file that describes this project,"] # [doc = " such as a `my-project/BUCK` file."] pub buildfile : AbsPathBuf , }
}