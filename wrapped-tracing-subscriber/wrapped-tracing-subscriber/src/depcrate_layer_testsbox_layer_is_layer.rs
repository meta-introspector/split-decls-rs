// Generated macro for box_layer_is_layer (function)
macro_rules! Depcrate_layer_testsbox_layer_is_layer {
() => {
// Module: crate::layer::tests
// Provides: {"box_layer_is_layer"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn box_layer_is_layer () { use alloc :: boxed :: Box ; let l : Box < dyn Layer < NoSubscriber > + Send + Sync > = Box :: new (NopLayer) ; assert_layer (& l) ; l . with_subscriber (NoSubscriber :: default ()) ; }
};
}
