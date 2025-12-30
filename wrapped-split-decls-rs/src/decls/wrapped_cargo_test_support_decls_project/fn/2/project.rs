use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: project");
# [doc = " Generates a project layout, see [`ProjectBuilder`]"] pub fn project () -> ProjectBuilder { ProjectBuilder :: new (paths :: root () . join ("foo")) }
}