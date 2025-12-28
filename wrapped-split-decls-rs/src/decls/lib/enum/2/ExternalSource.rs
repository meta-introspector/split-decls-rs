use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (PartialEq , Eq , Clone , Debug)] pub enum ExternalSource { # [doc = " No external source has to be loaded, since the `SourceFile` represents a local crate."] Unneeded , Foreign { kind : ExternalSourceKind , # [doc = " Index of the file inside metadata."] metadata_index : u32 , } , }