use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone)] pub enum NamedMatch { MatchedSeq (Box < [NamedMatch] >) , MatchedSingle (ParseNtResult) , }
}