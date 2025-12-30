// Generated macro for impl_68 (impl)
macro_rules! Depcrate_first_passimpl_68 {
() => {
// Module: crate::first_pass
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: InterfaceMixinDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } { let mixin_data = record . mixins . entry (self . identifier . 0) . or_default () ; mixin_data . partial = false ; mixin_data . definition_attributes = self . attributes . as_ref () ; mixin_data . stability = stability ; } for member in & self . members . body { member . first_pass (record , (self . identifier . 0 , stability)) ? ; } Ok (()) } }
};
}
