use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_4_Gcd_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type U4 = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U4GcdU4 = < < A as Gcd < B > > :: Output as Same < U4 > > :: Output ; assert_eq ! (< U4GcdU4 as Unsigned >:: to_u64 () , < U4 as Unsigned >:: to_u64 ()) ; }
}