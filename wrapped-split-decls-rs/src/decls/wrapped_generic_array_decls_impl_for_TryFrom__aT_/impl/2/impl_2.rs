use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T , N : ArrayLength > TryFrom < & 'a [T] > for & 'a GenericArray < T , N > { type Error = LengthError ; # [inline (always)] fn try_from (slice : & 'a [T]) -> Result < Self , Self :: Error > { GenericArray :: try_from_slice (slice) } }
}