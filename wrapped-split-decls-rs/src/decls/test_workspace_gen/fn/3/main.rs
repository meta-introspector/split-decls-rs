use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { let project_root = Path :: new ("../../") ; let (deps , patches) = generate_workspace_deps_from_project_root (& project_root) ? ; println ! ("Found {} workspace deps and {} patches" , deps . len () , patches . len ()) ; for (i , dep) in deps . iter () . enumerate () . take (10) { println ! ("Dep {}: {}" , i + 1 , dep) ; } Ok (()) }