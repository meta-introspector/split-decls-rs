// Generated macro for impl_67 (impl)
macro_rules! Depcrate_first_passimpl_67 {
() => {
// Module: crate::first_pass
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: interface :: AttributeInterfaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } record . interfaces . get_mut (ctx . 0) . unwrap () . attributes . push (AttributeInterfaceData { definition : self , stability : ctx . 1 , }) ; Ok (()) } }
};
}
