use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Max_5 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U3MaxU5 = < < A as Max < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U3MaxU5 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }
}