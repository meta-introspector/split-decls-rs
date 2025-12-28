use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < VariantDef > for ModuleDef { fn from (var : VariantDef) -> Self { match var { VariantDef :: Struct (t) => Adt :: from (t) . into () , VariantDef :: Union (t) => Adt :: from (t) . into () , VariantDef :: Variant (t) => t . into () , } } }