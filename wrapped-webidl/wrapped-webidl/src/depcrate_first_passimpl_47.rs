// Generated macro for impl_47 (impl)
macro_rules! Depcrate_first_passimpl_47 {
() => {
// Module: crate::first_pass
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > From < & 'a Argument < 'a > > for Arg < 'a > { fn from (arg : & 'a Argument < 'a >) -> Self { let (attributes , name , ty , optional , variadic) = match arg { Argument :: Single (single) => (& single . attributes , single . identifier . 0 , & single . type_ . type_ , single . optional . is_some () , false ,) , Argument :: Variadic (variadic) => (& variadic . attributes , variadic . identifier . 0 , & variadic . type_ , false , true ,) , } ; Self { attributes , name , ty , optional , variadic , } } }
};
}
