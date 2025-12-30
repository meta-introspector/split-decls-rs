// Generated macro for test_protocol (function)
macro_rules! Depcrate_test_objecttest_protocol {
() => {
// Module: crate::test_object
// Provides: {"test_protocol"}
// Dependencies: {}
# [test] # [cfg_attr (all (target_vendor = "apple" , not (target_arch = "aarch64")) , ignore = "has trouble linking")] fn test_protocol () { let obj = MyTestObject :: new () ; let proto : Retained < ProtocolObject < dyn MyTestProtocol > > = ProtocolObject :: from_retained (obj) ; assert_eq ! (proto . a () , 1) ; assert_eq ! (MyTestObject :: b () , 2) ; assert_eq ! (proto . c () . as_i32 () , 3) ; assert_eq ! (MyTestObject :: d () . as_i32 () , 4) ; assert_eq ! (proto . e () , 5) ; assert_eq ! (MyTestObject :: f () , 6) ; assert_eq ! (proto . g () . as_i32 () , 7) ; assert_eq ! (MyTestObject :: h () . as_i32 () , 8) ; let _obj : & ProtocolObject < dyn NSObjectProtocol > = ProtocolObject :: from_ref (& * proto) ; }
};
}
