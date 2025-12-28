use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Generates a project layout in given directory, see [`ProjectBuilder`]"] pub fn project_in (dir : impl AsRef < Path >) -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join (dir) . join ("foo")) }
}