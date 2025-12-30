// Generated macro for downcasts_to_subscriber (function)
macro_rules! Depcrate_layer_testsdowncasts_to_subscriber {
() => {
// Module: crate::layer::tests
// Provides: {"downcasts_to_subscriber"}
// Dependencies: {}
# [test] fn downcasts_to_subscriber () { let s = NopLayer . and_then (NopLayer) . and_then (NopLayer) . with_subscriber (StringSubscriber ("subscriber")) ; let subscriber = < dyn Subscriber > :: downcast_ref :: < StringSubscriber > (& s) . expect ("subscriber should downcast") ; assert_eq ! (subscriber . 0 , "subscriber") ; }
};
}
