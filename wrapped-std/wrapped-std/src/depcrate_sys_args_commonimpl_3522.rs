// Generated macro for impl_3522 (impl)
macro_rules! Depcrate_sys_args_commonimpl_3522 {
() => {
// Module: crate::sys::args::common
// Provides: {"impl_3522"}
// Dependencies: {}
impl DoubleEndedIterator for Args { # [inline] fn next_back (& mut self) -> Option < OsString > { self . iter . next_back () } # [inline] fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . iter . advance_back_by (n) } }
};
}
