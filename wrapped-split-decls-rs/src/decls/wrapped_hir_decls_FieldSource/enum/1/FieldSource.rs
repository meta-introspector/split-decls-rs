use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , PartialEq , Eq)] pub enum FieldSource { Named (ast :: RecordField) , Pos (ast :: TupleField) , }
}