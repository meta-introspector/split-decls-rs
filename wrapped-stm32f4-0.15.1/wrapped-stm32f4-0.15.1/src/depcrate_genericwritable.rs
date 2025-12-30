// Generated macro for Writable (trait)
macro_rules! Depcrate_genericWritable {
() => {
// Module: crate::generic
// Provides: {"Writable"}
// Dependencies: {}
# [doc = " Trait implemented by writeable registers."] # [doc = ""] # [doc = " This enables the  `write`, `write_with_zero` and `reset` methods."] # [doc = ""] # [doc = " Registers marked with `Readable` can be also `modify`'ed."] pub trait Writable : RegisterSpec { # [doc = " Writer type argument to `write`, et al."] type Writer : From < W < Self > > + core :: ops :: DerefMut < Target = W < Self > > ; }
};
}
