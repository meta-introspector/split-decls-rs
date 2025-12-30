// Generated macro for SharedPtrPair (type)
macro_rules! Depcrate_atomic_sharedSharedPtrPair {
() => {
// Module: crate::atomic_shared
// Provides: {"SharedPtrPair"}
// Dependencies: {}
# [doc = " A pair of [`Shared`] and [`Ptr`] of the same type."] pub type SharedPtrPair < 'g , T > = (Option < Shared < T > > , Ptr < 'g , T >) ;
};
}
