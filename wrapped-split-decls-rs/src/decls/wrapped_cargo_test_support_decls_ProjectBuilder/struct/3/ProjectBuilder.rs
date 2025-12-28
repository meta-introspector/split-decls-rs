use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Create a project to run tests against"] # [doc = ""] # [doc = " - Creates a [`basic_manifest`] if one isn't supplied"] # [doc = ""] # [doc = " To get started, see:"] # [doc = " - [`project`]"] # [doc = " - [`project_in`]"] # [doc = " - [`project_in_home`]"] # [doc = " - [`Project::from_template`]"] # [must_use] pub struct ProjectBuilder { root : Project , files : Vec < FileBuilder > , symlinks : Vec < SymlinkBuilder > , no_manifest : bool , }
}