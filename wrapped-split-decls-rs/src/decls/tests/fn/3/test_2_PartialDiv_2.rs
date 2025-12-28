use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_2_PartialDiv_2 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; type U1 = UInt < UTerm , B1 > ; # [allow (non_camel_case_types)] type U2PartialDivU2 = < < A as PartialDiv < B > > :: Output as Same < U1 > > :: Output ; assert_eq ! (< U2PartialDivU2 as Unsigned >:: to_u64 () , < U1 as Unsigned >:: to_u64 ()) ; }
}