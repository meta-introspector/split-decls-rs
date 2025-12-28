use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Pow_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N32 = NInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N2PowP5 = < < A as Pow < B > > :: Output as Same < N32 > > :: Output ; assert_eq ! (< N2PowP5 as Integer >:: to_i64 () , < N32 as Integer >:: to_i64 ()) ; }