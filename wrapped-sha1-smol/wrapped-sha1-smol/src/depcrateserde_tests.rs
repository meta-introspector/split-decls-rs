// Generated macro for serde_tests (module)
macro_rules! Depcrateserde_tests {
() => {
// Module: crate
// Provides: {"serde_tests"}
// Dependencies: {}
# [rustfmt :: skip] # [cfg (all (test , feature = "serde"))] mod serde_tests { extern crate std ; extern crate serde_json ; use self :: std :: prelude :: v1 :: * ; use crate :: { Sha1 , Digest } ; # [test] fn test_to_json () { let mut s = Sha1 :: new () ; s . update (b"Hello World!") ; let x = s . digest () ; let y = serde_json :: to_vec (& x) . unwrap () ; assert_eq ! (y , & b"\"2ef7bde608ce5404e97d5f042f95f89f1c232871\"" [..]) ; } # [test] fn test_from_json () { let y : Digest = serde_json :: from_str ("\"2ef7bde608ce5404e97d5f042f95f89f1c232871\"") . unwrap () ; assert_eq ! (y . to_string () , "2ef7bde608ce5404e97d5f042f95f89f1c232871") ; } }
};
}
