// Generated macro for impl_61 (impl)
macro_rules! Depcrate_parseimpl_61 {
() => {
// Module: crate::parse
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > fmt :: Debug for Handle < 'a , 'i , I , ModuleContents < 'a , 'i , I > > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?} => " , self . source_info ()) ? ; let mut first = true ; for x in self . all () { if ! first { write ! (f , " | ") ? ; } first = false ; fmt :: Debug :: fmt (& x , f) ? ; } Ok (()) } }
};
}
