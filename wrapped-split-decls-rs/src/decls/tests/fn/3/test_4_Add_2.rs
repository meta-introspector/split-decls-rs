use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Add_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U6 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > ; # [allow (non_camel_case_types)] type U4AddU2 = < < A as Add < B > > :: Output as Same < U6 > > :: Output ; assert_eq ! (< U4AddU2 as Unsigned >:: to_u64 () , < U6 as Unsigned >:: to_u64 ()) ; }
}