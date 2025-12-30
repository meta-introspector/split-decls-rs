// Generated macro for impl_26 (impl)
macro_rules! Depcrate_flagsimpl_26 {
() => {
// Module: crate::flags
// Provides: {"impl_26"}
// Dependencies: {}
impl Malloc { pub (crate) fn to_features (self) -> & 'static [& 'static str] { match self { Malloc :: System => & [] [..] , Malloc :: Mimalloc => & ["--features" , "mimalloc"] , Malloc :: Jemalloc => & ["--features" , "jemalloc"] , Malloc :: Dhat => & ["--features" , "dhat"] , } } }
};
}
