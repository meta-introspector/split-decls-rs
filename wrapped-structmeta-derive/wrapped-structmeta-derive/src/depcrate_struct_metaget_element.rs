// Generated macro for get_element (function)
macro_rules! Depcrate_struct_metaget_element {
() => {
// Module: crate::struct_meta
// Provides: {"get_element"}
// Dependencies: {}
fn get_element < 'a > (ty : & 'a Type , ns : & [& [& str]] , name : & str) -> Option < & 'a Type > { if let PathArguments :: AngleBracketed (args) = get_arguments_of (ty , ns , name) ? { if args . args . len () == 1 { if let GenericArgument :: Type (ty) = & args . args [0] { return Some (ty) ; } } } None }
};
}
