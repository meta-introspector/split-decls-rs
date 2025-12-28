use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable_Generic)] # [rustc_pass_by_value] pub struct Layout < 'a > (pub Interned < 'a , LayoutData < FieldIdx , VariantIdx > >) ;
}