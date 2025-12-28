use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > FieldBounds < 'a > { const ALL_SELF : FieldBounds < 'a > = FieldBounds :: All (& [TraitBound :: Slf]) ; const TRAILING_SELF : FieldBounds < 'a > = FieldBounds :: Trailing (& [TraitBound :: Slf]) ; }
}