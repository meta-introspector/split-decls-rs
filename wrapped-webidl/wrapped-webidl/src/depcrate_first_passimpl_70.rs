// Generated macro for impl_70 (impl)
macro_rules! Depcrate_first_passimpl_70 {
() => {
// Module: crate::first_pass
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: mixin :: MixinMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { match self { MixinMember :: Operation (op) => op . first_pass (record , ctx) , MixinMember :: Attribute (a) => a . first_pass (record , ctx) , MixinMember :: Const (a) => { if util :: is_chrome_only (& a . attributes) { return Ok (()) ; } record . mixins . get_mut (ctx . 0) . unwrap () . consts . push (a) ; Ok (()) } MixinMember :: Stringifier (_) => { log :: warn ! ("Unsupported WebIDL stringifier mixin member: {self:?}") ; Ok (()) } } } }
};
}
