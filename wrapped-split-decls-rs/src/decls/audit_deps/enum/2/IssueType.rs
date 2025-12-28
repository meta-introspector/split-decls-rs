use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] enum IssueType { MissingFromWorkspace , NotLocalPath , WorkspaceInheritance , MissingSubmodule , }
}