// Generated macro for impl_636 (impl)
macro_rules! Depcrate_viewsimpl_636 {
() => {
// Module: crate::views
// Provides: {"impl_636"}
// Dependencies: {}
impl ViewCaster { fn new < DbView : ? Sized + Any > (func : DatabaseDownCasterSig < DbView >) -> ViewCaster { ViewCaster { target_type_id : TypeId :: of :: < DbView > () , type_name : std :: any :: type_name :: < DbView > () , cast : unsafe { mem :: transmute :: < DatabaseDownCasterSig < DbView > , ErasedDatabaseDownCasterSig > (func) } , } } }
};
}
