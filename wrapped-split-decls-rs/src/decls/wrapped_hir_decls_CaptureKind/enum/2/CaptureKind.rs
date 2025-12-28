use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , PartialEq , Eq)] pub enum CaptureKind { SharedRef , UniqueSharedRef , MutableRef , Move , }
}