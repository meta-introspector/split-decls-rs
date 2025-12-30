// Generated macro for test_define_class_missing_protocol_method (function)
macro_rules! Depcrate_test_define_class_protocoltest_define_class_missing_protocol_method {
() => {
// Module: crate::test_define_class_protocol
// Provides: {"test_define_class_missing_protocol_method"}
// Dependencies: {}
# [test] # [cfg_attr (debug_assertions , should_panic = "must implement required protocol method -[NSCopying copyWithZone:]")] fn test_define_class_missing_protocol_method () { define_class ! (# [unsafe (super (NSObject))] # [name = "TestDefineClassMissingProtocolMethod"] struct Custom ; unsafe impl NSCopying for Custom { }) ; let _cls = Custom :: class () ; }
};
}
