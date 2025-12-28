use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Add_2 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U7 = UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > ; # [allow (non_camel_case_types)] type U5AddU2 = < < A as Add < B > > :: Output as Same < U7 > > :: Output ; assert_eq ! (< U5AddU2 as Unsigned >:: to_u64 () , < U7 as Unsigned >:: to_u64 ()) ; }
}