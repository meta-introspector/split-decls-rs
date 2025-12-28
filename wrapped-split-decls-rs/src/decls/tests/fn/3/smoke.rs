use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn smoke () { let value = 12u32 ; let reference = & value ; let tag = Tag2 :: B01 ; let ptr = TaggedRef :: new (reference , tag) ; assert_eq ! (ptr . tag () , tag) ; assert_eq ! (* ptr , 12) ; assert ! (ptr :: eq (ptr . pointer () , reference)) ; let copy = ptr ; let mut ptr = ptr ; ptr . set_tag (Tag2 :: B00) ; assert_eq ! (ptr . tag () , Tag2 :: B00) ; assert_eq ! (copy . tag () , tag) ; assert_eq ! (* copy , 12) ; assert ! (ptr :: eq (copy . pointer () , reference)) ; }