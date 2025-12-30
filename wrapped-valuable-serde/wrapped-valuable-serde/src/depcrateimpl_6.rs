// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < V > Serializable < V > where V : Valuable , { # [doc = " Creates a new `Serializable`."] pub fn new (v : V) -> Self { Self (v) } # [doc = " Returns a reference to the underlying value."] pub fn get_ref (& self) -> & V { & self . 0 } # [doc = " Returns a mutable reference to the underlying value."] pub fn get_mut (& mut self) -> & mut V { & mut self . 0 } # [doc = " Unwraps this `Serializable`, returning the underlying value."] pub fn into_inner (self) -> V { self . 0 } }
};
}
