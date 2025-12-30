// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: Inner :: * ; match self . inner { # [cfg (feature = "std")] Boxed (ref err) => err . fmt (f) , Msg (ref msg) => msg . fmt (f) , Fmt => fmt :: Error . fmt (f) , } } }
};
}
