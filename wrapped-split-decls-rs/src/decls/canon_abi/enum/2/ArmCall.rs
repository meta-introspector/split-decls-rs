use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " ABIs defined for 32-bit Arm"] # [derive (Copy , Clone , Debug)] # [derive (PartialOrd , Ord , PartialEq , Eq , Hash)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum ArmCall { Aapcs , CCmseNonSecureCall , CCmseNonSecureEntry , }
}