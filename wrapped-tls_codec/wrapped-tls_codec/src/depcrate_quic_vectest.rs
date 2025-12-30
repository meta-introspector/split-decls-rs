// Generated macro for test (module)
macro_rules! Depcrate_quic_vectest {
() => {
// Module: crate::quic_vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg (test)] mod test { use crate :: { SecretVLBytes , VLByteSlice , VLBytes } ; use std :: println ; # [test] fn test_debug () { let tests = [(vec ! [] , "b\"\"") , (vec ! [0x00] , "0x00") , (vec ! [0xAA] , "0xaa") , (vec ! [0xFF] , "0xff") , (vec ! [0x00 , 0x00] , "0x0000") , (vec ! [0x00 , 0xAA] , "0x00aa") , (vec ! [0x00 , 0xFF] , "0x00ff") , (vec ! [0xff , 0xff] , "0xffff") ,] ; for (test , expected) in tests . into_iter () { println ! ("\n# {test:?}") ; let expected_vl_byte_slice = format ! ("VLByteSlice {{ {expected} }}") ; let got = format ! ("{:?}" , VLByteSlice (& test)) ; println ! ("{got}") ; assert_eq ! (expected_vl_byte_slice , got) ; let expected_vl_bytes = format ! ("VLBytes {{ {expected} }}") ; let got = format ! ("{:?}" , VLBytes :: new (test . clone ())) ; println ! ("{got}") ; assert_eq ! (expected_vl_bytes , got) ; let expected_secret_vl_bytes = format ! ("SecretVLBytes {{ {expected} }}") ; let got = format ! ("{:?}" , SecretVLBytes :: new (test . clone ())) ; println ! ("{got}") ; assert_eq ! (expected_secret_vl_bytes , got) ; } } }
};
}
