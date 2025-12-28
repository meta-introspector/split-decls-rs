use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P4_Cmp_P1");
# [test] # [allow (non_snake_case)] fn test_P4_Cmp_P1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P4CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P4CmpP1 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}