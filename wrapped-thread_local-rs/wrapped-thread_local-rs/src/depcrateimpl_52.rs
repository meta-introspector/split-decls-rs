// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < T : Send + Default > ThreadLocal < T > { # [doc = " Returns the element for the current thread, or creates a default one if"] # [doc = " it doesn't exist."] pub fn get_or_default (& self) -> & T { self . get_or (Default :: default) } }
};
}
