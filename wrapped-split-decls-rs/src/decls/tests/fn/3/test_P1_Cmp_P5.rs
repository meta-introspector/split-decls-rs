use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P1_Cmp_P5");
# [test] # [allow (non_snake_case)] fn test_P1_Cmp_P5 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P1CmpP5 = < A as Cmp < B > > :: Output ; assert_eq ! (< P1CmpP5 as Ord >:: to_ordering () , Ordering :: Less) ; }
}