use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P3_Cmp_P2");
# [test] # [allow (non_snake_case)] fn test_P3_Cmp_P2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P3CmpP2 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpP2 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}