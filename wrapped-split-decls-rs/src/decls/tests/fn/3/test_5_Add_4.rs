use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_5_Add_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U9 = UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > ; # [allow (non_camel_case_types)] type U5AddU4 = < < A as Add < B > > :: Output as Same < U9 > > :: Output ; assert_eq ! (< U5AddU4 as Unsigned >:: to_u64 () , < U9 as Unsigned >:: to_u64 ()) ; }
}