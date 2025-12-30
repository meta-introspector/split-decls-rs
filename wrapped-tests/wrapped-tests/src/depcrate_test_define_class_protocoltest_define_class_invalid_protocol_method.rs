// Generated macro for test_define_class_invalid_protocol_method (function)
macro_rules! Depcrate_test_define_class_protocoltest_define_class_invalid_protocol_method {
() => {
// Module: crate::test_define_class_protocol
// Provides: {"test_define_class_invalid_protocol_method"}
// Dependencies: {}
# [test] fn test_define_class_invalid_protocol_method () { define_class ! (# [unsafe (super (NSObject))] # [name = "TestDefineClassInvalidProtocolMethod"] struct Custom ; unsafe impl NSCopying for Custom { # [unsafe (method (copyWithZone :))] fn copy_with_zone (& self , _zone : * const NSZone) -> u8 { 42 } }) ; let _cls = Custom :: class () ; }
};
}
