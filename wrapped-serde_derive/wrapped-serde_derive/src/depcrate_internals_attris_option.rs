// Generated macro for is_option (function)
macro_rules! Depcrate_internals_attris_option {
() => {
// Module: crate::internals::attr
// Provides: {"is_option"}
// Dependencies: {}
fn is_option (ty : & syn :: Type , elem : fn (& syn :: Type) -> bool) -> bool { let path = match ungroup (ty) { syn :: Type :: Path (ty) => & ty . path , _ => { return false ; } } ; let Some (seg) = path . segments . last () else { return false ; } ; let args = match & seg . arguments { syn :: PathArguments :: AngleBracketed (bracketed) => & bracketed . args , _ => { return false ; } } ; seg . ident == "Option" && args . len () == 1 && match & args [0] { syn :: GenericArgument :: Type (arg) => elem (arg) , _ => false , } }
};
}
