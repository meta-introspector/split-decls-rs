use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_BitOr_3 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; type U3 = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U3BitOrU3 = < < A as BitOr < B > > :: Output as Same < U3 > > :: Output ; assert_eq ! (< U3BitOrU3 as Unsigned >:: to_u64 () , < U3 as Unsigned >:: to_u64 ()) ; }