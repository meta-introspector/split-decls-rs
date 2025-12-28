use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P3_Cmp_N2");
# [test] # [allow (non_snake_case)] fn test_P3_Cmp_N2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P3CmpN2 = < A as Cmp < B > > :: Output ; assert_eq ! (< P3CmpN2 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}