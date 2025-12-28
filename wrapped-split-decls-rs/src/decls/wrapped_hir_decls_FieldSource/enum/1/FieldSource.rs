use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , PartialEq , Eq)] pub enum FieldSource { Named (ast :: RecordField) , Pos (ast :: TupleField) , }