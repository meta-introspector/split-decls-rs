// Generated macro for impl_60 (impl)
macro_rules! Depcrate_first_passimpl_60 {
() => {
// Module: crate::first_pass
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , (& 'src str , ApiStability) > for weedle :: interface :: InterfaceMember < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , ctx : (& 'src str , ApiStability) ,) -> Result < () > { match self { InterfaceMember :: Attribute (attr) => attr . first_pass (record , ctx) , InterfaceMember :: Operation (op) => op . first_pass (record , ctx) , InterfaceMember :: Const (const_) => { if util :: is_chrome_only (& const_ . attributes) { return Ok (()) ; } record . interfaces . get_mut (ctx . 0) . unwrap () . consts . push (ConstData { definition : const_ , stability : ctx . 1 , }) ; Ok (()) } InterfaceMember :: Constructor (constr) => constr . first_pass (record , ctx) , InterfaceMember :: Maplike (ml) => ml . first_pass (record , ctx) , InterfaceMember :: Setlike (sl) => sl . first_pass (record , ctx) , InterfaceMember :: Iterable (iterable) => iterable . first_pass (record , ctx) , InterfaceMember :: AsyncIterable (iterable) => iterable . first_pass (record , ctx) , InterfaceMember :: Stringifier (_) => { log :: warn ! ("Unsupported WebIDL Stringifier interface member: {self:?}") ; Ok (()) } } } }
};
}
