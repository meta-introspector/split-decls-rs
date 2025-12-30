// Generated macro for impl_110 (impl)
macro_rules! Depcrate_errorimpl_110 {
() => {
// Module: crate::error
// Provides: {"impl_110"}
// Dependencies: {}
impl core :: fmt :: Display for StrContext { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Label (name) => write ! (f , "invalid {name}") , Self :: Expected (value) => write ! (f , "expected {value}") , } } }
};
}
