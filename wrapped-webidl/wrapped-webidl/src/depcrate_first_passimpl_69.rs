// Generated macro for impl_69 (impl)
macro_rules! Depcrate_first_passimpl_69 {
() => {
// Module: crate::first_pass
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: PartialInterfaceMixinDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } record . mixins . entry (self . identifier . 0) . or_insert_with (| | MixinData { partial : true , stability , .. Default :: default () }) ; for member in & self . members . body { member . first_pass (record , (self . identifier . 0 , stability)) ? ; } Ok (()) } }
};
}
