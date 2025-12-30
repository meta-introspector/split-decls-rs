// Generated macro for impl_72 (impl)
macro_rules! Depcrate_first_passimpl_72 {
() => {
// Module: crate::first_pass
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: mixin :: AttributeMixinMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } record . mixins . get_mut (ctx . 0) . unwrap () . attributes . push (AttributeMixinData { definition : self , stability : ctx . 1 , }) ; Ok (()) } }
};
}
