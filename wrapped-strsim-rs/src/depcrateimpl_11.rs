// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Display for StrSimError { fn fmt (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { let text = match self { StrSimError :: DifferentLengthArgs => "Differing length arguments provided" , } ; write ! (fmt , "{}" , text) } }
};
}
