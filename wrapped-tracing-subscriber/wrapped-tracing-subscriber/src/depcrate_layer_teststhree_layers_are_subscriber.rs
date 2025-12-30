// Generated macro for three_layers_are_subscriber (function)
macro_rules! Depcrate_layer_teststhree_layers_are_subscriber {
() => {
// Module: crate::layer::tests
// Provides: {"three_layers_are_subscriber"}
// Dependencies: {}
# [test] fn three_layers_are_subscriber () { let s = NopLayer . and_then (NopLayer) . and_then (NopLayer) . with_subscriber (NoSubscriber :: default ()) ; assert_subscriber (s) }
};
}
