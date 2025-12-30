// Generated macro for ViewCaster (struct)
macro_rules! Depcrate_viewsViewCaster {
() => {
// Module: crate::views
// Provides: {"ViewCaster"}
// Dependencies: {}
# [derive (Copy , Clone)] struct ViewCaster { # [doc = " The id of the target type `dyn DbView` that we can cast to."] target_type_id : TypeId , # [doc = " The name of the target type `dyn DbView` that we can cast to."] type_name : & 'static str , # [doc = " Type-erased function pointer that downcasts to `dyn DbView`."] cast : ErasedDatabaseDownCasterSig , }
};
}
