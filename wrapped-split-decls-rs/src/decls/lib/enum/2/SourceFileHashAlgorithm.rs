use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash , Encodable , Decodable)] # [derive (HashStable_Generic)] pub enum SourceFileHashAlgorithm { Md5 , Sha1 , Sha256 , Blake3 , }
}