use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " The default VersionReq is the same as [`VersionReq::STAR`]."] impl Default for VersionReq { fn default () -> Self { VersionReq :: STAR } }
}