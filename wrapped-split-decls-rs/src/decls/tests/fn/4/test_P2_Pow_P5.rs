use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_P2_Pow_P5");
# [test] # [allow (non_snake_case)] fn test_P2_Pow_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P32 = PInt < UInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P2PowP5 = < < A as Pow < B > > :: Output as Same < P32 > > :: Output ; assert_eq ! (< P2PowP5 as Integer >:: to_i64 () , < P32 as Integer >:: to_i64 ()) ; }
}