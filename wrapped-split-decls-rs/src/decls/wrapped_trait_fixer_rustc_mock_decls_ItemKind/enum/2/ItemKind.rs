use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemKind < 'tcx > { Struct (PhantomData < & 'tcx () >) , Enum (PhantomData < & 'tcx () >) , Union (PhantomData < & 'tcx () >) , }
}