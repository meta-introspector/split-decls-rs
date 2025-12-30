// Generated macro for impl_18 (impl)
macro_rules! Depcrate_bodytimpl_18 {
() => {
// Module: crate::bodyt
// Provides: {"impl_18"}
// Dependencies: {}
impl From < Option < Bytes > > for Body { fn from (opt : Option < Bytes >) -> Self { match opt { Some (b) => b . into () , None => Body :: empty () , } } }
};
}
