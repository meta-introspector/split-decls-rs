// Generated macro for get_arguments_of (function)
macro_rules! Depcrate_syn_utilsget_arguments_of {
() => {
// Module: crate::syn_utils
// Provides: {"get_arguments_of"}
// Dependencies: {}
pub fn get_arguments_of < 'a > (ty : & 'a Type , ns : & [& [& str]] , name : & str) -> Option < & 'a PathArguments > { if let Type :: Path (ty) = ty { if ty . qself . is_some () { return None ; } let ss = & ty . path . segments ; if let Some (last) = ty . path . segments . last () { if last . ident != name { return None ; } return if ns . iter () . any (| ns | is_match_ns (ss , ns)) { Some (& last . arguments) } else { None } ; } } None }
};
}
