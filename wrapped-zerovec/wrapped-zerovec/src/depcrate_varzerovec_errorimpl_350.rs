// Generated macro for impl_350 (impl)
macro_rules! Depcrate_varzerovec_errorimpl_350 {
() => {
// Module: crate::varzerovec::error
// Provides: {"impl_350"}
// Dependencies: {}
impl Display for VarZeroVecFormatError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Metadata => write ! (f , "VarZeroVecFormatError: metadata") , Self :: Values (e) => write ! (f , "VarZeroVecFormatError: {e}") , } } }
};
}
