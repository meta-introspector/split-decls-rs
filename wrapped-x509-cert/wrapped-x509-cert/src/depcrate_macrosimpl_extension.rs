// Generated macro for impl_extension (macro)
macro_rules! Depcrate_macrosimpl_extension {
() => {
// Module: crate::macros
// Provides: {"impl_extension"}
// Dependencies: {}
# [doc = " Implements the AsExtension traits for every defined Extension paylooad"] macro_rules ! impl_extension { ($ newtype : ty) => { impl_extension ! ($ newtype , critical = false) ; } ; ($ newtype : ty , critical = $ critical : expr) => { impl crate :: ext :: AsExtension for $ newtype { fn critical (& self , _subject : & crate :: name :: Name , _extensions : & [crate :: ext :: Extension] ,) -> bool { $ critical } } } ; }
};
}
