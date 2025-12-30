// Generated macro for impl_630 (impl)
macro_rules! Depcrateimpl_630 {
() => {
// Module: crate
// Provides: {"impl_630"}
// Dependencies: {}
impl Crate { # [doc = " The list of foreign modules in this crate."] pub fn foreign_modules (& self) -> Vec < ForeignModuleDef > { with (| cx | cx . foreign_modules (self . id)) } # [doc = " The list of traits declared in this crate."] pub fn trait_decls (& self) -> TraitDecls { with (| cx | cx . trait_decls (self . id)) } # [doc = " The list of trait implementations in this crate."] pub fn trait_impls (& self) -> ImplTraitDecls { with (| cx | cx . trait_impls (self . id)) } # [doc = " Return a list of function definitions from this crate independent on their visibility."] pub fn fn_defs (& self) -> Vec < FnDef > { with (| cx | cx . crate_functions (self . id)) } # [doc = " Return a list of static items defined in this crate independent on their visibility."] pub fn statics (& self) -> Vec < StaticDef > { with (| cx | cx . crate_statics (self . id)) } }
};
}
