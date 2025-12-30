// Generated macro for test_define_class_extra_protocol_method (function)
macro_rules! Depcrate_test_define_class_protocoltest_define_class_extra_protocol_method {
() => {
// Module: crate::test_define_class_protocol
// Provides: {"test_define_class_extra_protocol_method"}
// Dependencies: {}
# [test] # [cfg_attr (debug_assertions , should_panic = "failed overriding protocol method -[NSCopying someOtherMethod]: method not found")] fn test_define_class_extra_protocol_method () { define_class ! (# [unsafe (super (NSObject))] # [name = "TestDefineClassExtraProtocolMethod"] struct Custom ; unsafe impl NSCopying for Custom { # [unsafe (method_id (copyWithZone :))] fn copy_with_zone (& self , _zone : * const NSZone) -> Retained < Self > { unimplemented ! () } # [unsafe (method (someOtherMethod))] fn some_other_method (& self) { } }) ; let _cls = Custom :: class () ; }
};
}
