// Generated macro for impl_736 (impl)
macro_rules! Depcrate_specimpl_736 {
() => {
// Module: crate::spec
// Provides: {"impl_736"}
// Dependencies: {}
impl PanicStrategy { pub const fn desc_symbol (& self) -> Symbol { match * self { PanicStrategy :: Unwind => sym :: unwind , PanicStrategy :: Abort => sym :: abort , } } pub const fn all () -> [Symbol ; 2] { [Self :: Abort . desc_symbol () , Self :: Unwind . desc_symbol ()] } }
};
}
