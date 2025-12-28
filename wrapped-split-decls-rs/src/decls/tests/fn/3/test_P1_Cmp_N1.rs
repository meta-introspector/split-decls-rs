use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P1_Cmp_N1");
# [test] # [allow (non_snake_case)] fn test_P1_Cmp_N1 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1CmpN1 = < A as Cmp < B > > :: Output ; assert_eq ! (< P1CmpN1 as Ord >:: to_ordering () , Ordering :: Greater) ; }
}