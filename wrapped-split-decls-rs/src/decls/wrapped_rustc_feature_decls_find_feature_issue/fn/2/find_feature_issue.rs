use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn find_feature_issue (feature : Symbol , issue : GateIssue) -> Option < NonZero < u32 > > { match issue { GateIssue :: Language => find_lang_feature_issue (feature) , GateIssue :: Library (lib) => lib , } }
}