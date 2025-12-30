// Generated macro for impl_671 (impl)
macro_rules! Depcrate_zalsaimpl_671 {
() => {
// Module: crate::zalsa
// Provides: {"impl_671"}
// Dependencies: {}
# [doc = " All fields on Zalsa are locked behind [`Mutex`]es and [`RwLock`]s and cannot enter"] # [doc = " inconsistent states. The contents of said fields are largely ID mappings, with the exception"] # [doc = " of [`Runtime::dependency_graph`]. However, [`Runtime::dependency_graph`] does not"] # [doc = " invoke any queries and as such there will be no panic from code downstream of Salsa. It can only"] # [doc = " panic if an assertion inside of Salsa fails."] impl RefUnwindSafe for Zalsa { }
};
}
