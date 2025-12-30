// Generated macro for Readable (trait)
macro_rules! Depcrate_genericReadable {
() => {
// Module: crate::generic
// Provides: {"Readable"}
// Dependencies: {}
# [doc = " Trait implemented by readable registers to enable the `read` method."] # [doc = ""] # [doc = " Registers marked with `Writable` can be also `modify`'ed."] pub trait Readable : RegisterSpec { # [doc = " Result from a call to `read` and argument to `modify`."] type Reader : From < R < Self > > + core :: ops :: Deref < Target = R < Self > > ; }
};
}
