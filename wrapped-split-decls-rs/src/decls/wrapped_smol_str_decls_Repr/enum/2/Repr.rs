use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug)] enum Repr { Inline { len : InlineSize , buf : [u8 ; INLINE_CAP] , } , Static (& 'static str) , Heap (Arc < str >) , }
}