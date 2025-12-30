// Generated macro for IntermediateIterator (struct)
macro_rules! Depcrate_verify_certIntermediateIterator {
() => {
// Module: crate::verify_cert
// Provides: {"IntermediateIterator"}
// Dependencies: {}
# [doc = " Iterator over a path's intermediate certificates."] # [doc = ""] # [doc = " Implements [`DoubleEndedIterator`] so it can be traversed in both directions."] pub struct IntermediateIterator < 'a > { # [doc = " Invariant: all of these `Option`s are `Some`."] intermediates : & 'a [Option < Cert < 'a > >] , }
};
}
