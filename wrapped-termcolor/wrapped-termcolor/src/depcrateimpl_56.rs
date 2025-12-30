// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < W : Write > Ansi < W > { # [doc = " Create a new writer that satisfies `WriteColor` using standard ANSI"] # [doc = " escape sequences."] pub fn new (wtr : W) -> Ansi < W > { Ansi (wtr) } # [doc = " Consume this `Ansi` value and return the inner writer."] pub fn into_inner (self) -> W { self . 0 } # [doc = " Return a reference to the inner writer."] pub fn get_ref (& self) -> & W { & self . 0 } # [doc = " Return a mutable reference to the inner writer."] pub fn get_mut (& mut self) -> & mut W { & mut self . 0 } }
};
}
