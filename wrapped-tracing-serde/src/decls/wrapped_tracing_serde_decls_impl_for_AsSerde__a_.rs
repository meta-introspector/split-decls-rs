use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> AsSerde<'a> for FieldSet {
    type Serializable = SerializeFieldSet<'a>;
    fn as_serde(&'a self) -> Self::Serializable {
        SerializeFieldSet(self)
    }
}
