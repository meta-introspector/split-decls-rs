use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum CallableKind < 'db > { Function (Function) , TupleStruct (Struct) , TupleEnumVariant (Variant) , Closure (Closure < 'db >) , FnPtr , FnImpl (FnTrait) , }
}