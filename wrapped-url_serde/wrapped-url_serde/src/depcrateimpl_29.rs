// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'de , T > fmt :: Debug for Serde < T > where T : fmt :: Debug , De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { self . 0 . fmt (formatter) } }
};
}
