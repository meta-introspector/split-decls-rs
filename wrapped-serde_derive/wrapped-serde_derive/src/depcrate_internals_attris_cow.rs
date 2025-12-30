// Generated macro for is_cow (function)
macro_rules! Depcrate_internals_attris_cow {
() => {
// Module: crate::internals::attr
// Provides: {"is_cow"}
// Dependencies: {}
fn is_cow (ty : & syn :: Type , elem : fn (& syn :: Type) -> bool) -> bool { let path = match ungroup (ty) { syn :: Type :: Path (ty) => & ty . path , _ => { return false ; } } ; let Some (seg) = path . segments . last () else { return false ; } ; let args = match & seg . arguments { syn :: PathArguments :: AngleBracketed (bracketed) => & bracketed . args , _ => { return false ; } } ; seg . ident == "Cow" && args . len () == 2 && match (& args [0] , & args [1]) { (syn :: GenericArgument :: Lifetime (_) , syn :: GenericArgument :: Type (arg)) => elem (arg) , _ => false , } }
};
}
