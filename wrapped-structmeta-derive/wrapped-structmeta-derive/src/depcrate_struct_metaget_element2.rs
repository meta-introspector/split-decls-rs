// Generated macro for get_element2 (function)
macro_rules! Depcrate_struct_metaget_element2 {
() => {
// Module: crate::struct_meta
// Provides: {"get_element2"}
// Dependencies: {}
fn get_element2 < 'a > (ty : & 'a Type , ns : & [& [& str]] , name : & str) -> Option < (& 'a Type , & 'a Type) > { if let PathArguments :: AngleBracketed (args) = get_arguments_of (ty , ns , name) ? { if args . args . len () == 2 { if let (GenericArgument :: Type (ty0) , GenericArgument :: Type (ty1)) = (& args . args [0] , & args . args [1]) { return Some ((ty0 , ty1)) ; } } } None }
};
}
