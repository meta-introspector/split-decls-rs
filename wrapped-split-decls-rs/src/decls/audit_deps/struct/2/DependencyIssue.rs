use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct DependencyIssue { crate_name : String , dep_name : String , issue_type : IssueType , current_spec : String , suggested_fix : Option < String > , }
}