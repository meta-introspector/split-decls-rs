use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T : PartialOrd > PartialOrd for Interned < 'a , T > { fn partial_cmp (& self , other : & Interned < 'a , T >) -> Option < Ordering > { if ptr :: eq (self . 0 , other . 0) { Some (Ordering :: Equal) } else { let res = self . 0 . partial_cmp (other . 0) ; debug_assert_ne ! (res , Some (Ordering :: Equal)) ; res } } }