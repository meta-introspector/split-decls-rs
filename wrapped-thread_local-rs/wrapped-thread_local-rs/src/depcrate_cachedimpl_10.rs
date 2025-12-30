// Generated macro for impl_10 (impl)
macro_rules! Depcrate_cachedimpl_10 {
() => {
// Module: crate::cached
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Send + Default > CachedThreadLocal < T > { # [doc = " Returns the element for the current thread, or creates a default one if"] # [doc = " it doesn't exist."] pub fn get_or_default (& self) -> & T { self . get_or (T :: default) } }
};
}
