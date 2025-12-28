use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type MaxArrayLengthP1 = typenum :: Shleft < typenum :: U1 , typenum :: Shleft < typenum :: U < { mem :: size_of :: < usize > () } > , typenum :: U3 > , > ;
}