use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for GenericArray < T , N > where GenericArray < U , N > : GenericSequence < U , Length = N > , { type Mapped = GenericArray < U , N > ; }
}