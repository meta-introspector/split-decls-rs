use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct MetavarSpansMap (FreezeLock < UnordMap < Span , (Span , bool) > >) ;
}