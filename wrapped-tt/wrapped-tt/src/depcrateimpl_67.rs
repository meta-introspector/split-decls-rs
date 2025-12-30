// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl < S : fmt :: Display + Copy > fmt :: Display for TopSubtree < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . view () , f) } }
};
}
