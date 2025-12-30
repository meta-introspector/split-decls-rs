// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < W : Write > NoColor < W > { # [doc = " Create a new writer that satisfies `WriteColor` but drops all color"] # [doc = " information."] pub fn new (wtr : W) -> NoColor < W > { NoColor (wtr) } # [doc = " Consume this `NoColor` value and return the inner writer."] pub fn into_inner (self) -> W { self . 0 } # [doc = " Return a reference to the inner writer."] pub fn get_ref (& self) -> & W { & self . 0 } # [doc = " Return a mutable reference to the inner writer."] pub fn get_mut (& mut self) -> & mut W { & mut self . 0 } }
};
}
