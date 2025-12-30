// Generated macro for impl_76 (impl)
macro_rules! Depcrate_first_passimpl_76 {
() => {
// Module: crate::first_pass
// Provides: {"impl_76"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: namespace :: NamespaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { match self { weedle :: namespace :: NamespaceMember :: Const (const_) => { record . namespaces . get_mut (ctx . 0) . unwrap () . consts . push (ConstNamespaceData { definition : const_ , stability : ctx . 1 , }) ; Ok (()) } weedle :: namespace :: NamespaceMember :: Operation (op) => op . first_pass (record , ctx) , _ => Ok (()) , } } }
};
}
