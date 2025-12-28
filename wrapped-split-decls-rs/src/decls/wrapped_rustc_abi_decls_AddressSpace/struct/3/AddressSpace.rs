use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An identifier that specifies the address space that some operation"] # [doc = " should operate on. Special address spaces have an effect on code generation,"] # [doc = " depending on the target and the address spaces it implements."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub struct AddressSpace (pub u32) ;
}