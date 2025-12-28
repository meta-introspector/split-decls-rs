use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct TransitiveRelation < T > { builder : Frozen < TransitiveRelationBuilder < T > > , closure : Frozen < BitMatrix < usize , usize > > , }
}