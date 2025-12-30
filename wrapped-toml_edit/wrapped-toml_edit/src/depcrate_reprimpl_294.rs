// Generated macro for impl_294 (impl)
macro_rules! Depcrate_reprimpl_294 {
() => {
// Module: crate::repr
// Provides: {"impl_294"}
// Dependencies: {}
impl < T > std :: fmt :: Debug for Formatted < T > where T : std :: fmt :: Debug , { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut d = formatter . debug_struct ("Formatted") ; d . field ("value" , & self . value) ; match & self . repr { Some (r) => d . field ("repr" , r) , None => d . field ("repr" , & "default") , } ; d . field ("decor" , & self . decor) ; d . finish () } }
};
}
