use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for GetDisjointMutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let msg = match self { GetDisjointMutError :: IndexVacant => "an index is vacant" , GetDisjointMutError :: IndexOutOfBounds => "an index is out of bounds" , GetDisjointMutError :: OverlappingIndices => "there were overlapping indices" , } ; fmt :: Display :: fmt (msg , f) } }
}