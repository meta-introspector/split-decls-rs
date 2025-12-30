// Generated macro for test_define_class_protocol (function)
macro_rules! Depcrate_test_define_class_protocoltest_define_class_protocol {
() => {
// Module: crate::test_define_class_protocol
// Provides: {"test_define_class_protocol"}
// Dependencies: {}
# [test] fn test_define_class_protocol () { define_class ! (# [unsafe (super (NSObject))] # [name = "TestDefineClassProtocolNotFound"] struct Custom ; unsafe impl NSCopying for Custom { # [unsafe (method_id (copyWithZone :))] fn copy_with_zone (& self , _zone : * const NSZone) -> Retained < Self > { unimplemented ! () } }) ; let cls = Custom :: class () ; assert ! (cls . conforms_to (< dyn NSCopying >:: protocol () . unwrap ())) ; }
};
}
