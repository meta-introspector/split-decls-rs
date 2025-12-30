// Generated macro for test_define_class_invalid_method (function)
macro_rules! Depcrate_test_define_class_protocoltest_define_class_invalid_method {
() => {
// Module: crate::test_define_class_protocol
// Provides: {"test_define_class_invalid_method"}
// Dependencies: {}
# [test] # [cfg_attr (debug_assertions , should_panic = "defined invalid method -[TestDefineClassInvalidMethod description]: expected return to have type code '@', but found 'v'")] fn test_define_class_invalid_method () { define_class ! (# [unsafe (super (NSObject))] # [name = "TestDefineClassInvalidMethod"] struct Custom ; impl Custom { # [unsafe (method (description))] fn description (& self) { } }) ; let _cls = Custom :: class () ; }
};
}
