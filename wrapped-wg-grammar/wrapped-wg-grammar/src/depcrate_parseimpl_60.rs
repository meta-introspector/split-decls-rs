// Generated macro for impl_60 (impl)
macro_rules! Depcrate_parseimpl_60 {
() => {
// Module: crate::parse
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > fmt :: Debug for ModuleContents < 'a , 'i , I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut d = f . debug_struct ("ModuleContents") ; d . field ("items" , & self . items) ; d . finish () } }
};
}
