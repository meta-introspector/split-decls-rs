// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : MacroResult > MacroResult for Result < T > { fn into_result (self) -> Result < TokenStream > { match self { Ok (v) => v . into_result () , Err (err) => Err (err) , } } }
};
}
