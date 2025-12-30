// Generated macro for two_layers_are_subscriber (function)
macro_rules! Depcrate_layer_teststwo_layers_are_subscriber {
() => {
// Module: crate::layer::tests
// Provides: {"two_layers_are_subscriber"}
// Dependencies: {}
# [test] fn two_layers_are_subscriber () { let s = NopLayer . and_then (NopLayer) . with_subscriber (NoSubscriber :: default ()) ; assert_subscriber (s) }
};
}
