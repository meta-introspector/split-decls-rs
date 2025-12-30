// Generated macro for impl_55 (impl)
macro_rules! Depcrate_parseimpl_55 {
() => {
// Module: crate::parse
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input > fmt :: Debug for Handle < 'a , 'i , I , Expr < 'a , 'i , I > > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?} => " , self . source_info ()) ? ; let mut first = true ; for x in self . all () { if ! first { write ! (f , " | ") ? ; } first = false ; fmt :: Debug :: fmt (& x , f) ? ; } Ok (()) } }
};
}
