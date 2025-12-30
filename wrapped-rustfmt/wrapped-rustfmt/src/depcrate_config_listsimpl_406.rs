// Generated macro for impl_406 (impl)
macro_rules! Depcrate_config_listsimpl_406 {
() => {
// Module: crate::config::lists
// Provides: {"impl_406"}
// Dependencies: {}
impl SeparatorPlace { pub fn is_front (self) -> bool { self == SeparatorPlace :: Front } pub fn is_back (self) -> bool { self == SeparatorPlace :: Back } pub fn from_tactic (default : SeparatorPlace , tactic : DefinitiveListTactic , sep : & str ,) -> SeparatorPlace { match tactic { DefinitiveListTactic :: Vertical => default , _ => { if sep == "," { SeparatorPlace :: Back } else { default } } } } }
};
}
