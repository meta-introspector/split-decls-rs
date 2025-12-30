// Generated macro for has_plf_stubs (module)
macro_rules! Depcrate_filterhas_plf_stubs {
() => {
// Module: crate::filter
// Provides: {"has_plf_stubs"}
// Dependencies: {}
# [doc = " Stub implementations of the per-layer-filter detection functions for when the"] # [doc = " `registry` feature is disabled."] # [cfg (not (all (feature = "registry" , feature = "std")))] mod has_plf_stubs { pub (crate) fn is_plf_downcast_marker (_ : core :: any :: TypeId) -> bool { false } # [doc = " Does a type implementing `Subscriber` contain any per-layer filters?"] pub (crate) fn subscriber_has_plf < S > (_ : & S) -> bool where S : tracing_core :: Subscriber , { false } # [doc = " Does a type implementing `Layer` contain any per-layer filters?"] pub (crate) fn layer_has_plf < L , S > (_ : & L) -> bool where L : crate :: Layer < S > , S : tracing_core :: Subscriber , { false } }
};
}
