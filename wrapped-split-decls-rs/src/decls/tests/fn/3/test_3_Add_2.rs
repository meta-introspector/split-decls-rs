use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_3_Add_2 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U5 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U3AddU2 = < < A as Add < B > > :: Output as Same < U5 > > :: Output ; assert_eq ! (< U3AddU2 as Unsigned >:: to_u64 () , < U5 as Unsigned >:: to_u64 ()) ; }
}