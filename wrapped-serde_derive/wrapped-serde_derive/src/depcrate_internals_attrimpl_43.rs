// Generated macro for impl_43 (impl)
macro_rules! Depcrate_internals_attrimpl_43 {
() => {
// Module: crate::internals::attr
// Provides: {"impl_43"}
// Dependencies: {}
impl RenameAllRules { # [doc = " Returns a new `RenameAllRules` with the individual rules of `self` and"] # [doc = " `other_rules` joined by `RenameRules::or`."] pub fn or (self , other_rules : Self) -> Self { Self { serialize : self . serialize . or (other_rules . serialize) , deserialize : self . deserialize . or (other_rules . deserialize) , } } }
};
}
