// Generated macro for impl_639 (impl)
macro_rules! Depcrate_viewsimpl_639 {
() => {
// Module: crate::views
// Provides: {"impl_639"}
// Dependencies: {}
impl ViewCaster { fn new < DbView : ? Sized + Any > (func : DatabaseDownCasterSig < DbView >) -> ViewCaster { ViewCaster { target_type_id : TypeId :: of :: < DbView > () , type_name : std :: any :: type_name :: < DbView > () , cast : unsafe { mem :: transmute :: < DatabaseDownCasterSig < DbView > , ErasedDatabaseDownCasterSig > (func) } , } } }
};
}
