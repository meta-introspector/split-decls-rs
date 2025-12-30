// Generated macro for tests (module)
macro_rules! Depcrate_layer_fntests {
() => {
// Module: crate::layer_fn
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: { format , string :: ToString } ; # [allow (dead_code)] # [test] fn layer_fn_has_useful_debug_impl () { struct WrappedService < S > { inner : S , } let layer = layer_fn (| svc | WrappedService { inner : svc }) ; let _svc = layer . layer ("foo") ; assert_eq ! ("LayerFn { f: tower_layer::layer_fn::tests::layer_fn_has_useful_debug_impl::{{closure}} }" . to_string () , format ! ("{layer:?}") ,) ; } }
};
}
