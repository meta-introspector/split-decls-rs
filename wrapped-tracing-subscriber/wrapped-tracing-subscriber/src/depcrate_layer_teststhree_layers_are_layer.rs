// Generated macro for three_layers_are_layer (function)
macro_rules! Depcrate_layer_teststhree_layers_are_layer {
() => {
// Module: crate::layer::tests
// Provides: {"three_layers_are_layer"}
// Dependencies: {}
# [test] fn three_layers_are_layer () { let layers = NopLayer . and_then (NopLayer) . and_then (NopLayer) ; assert_layer (& layers) ; let _ = layers . with_subscriber (NoSubscriber :: default ()) ; }
};
}
