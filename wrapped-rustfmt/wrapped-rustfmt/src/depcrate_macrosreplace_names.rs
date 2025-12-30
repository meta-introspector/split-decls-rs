// Generated macro for replace_names (function)
macro_rules! Depcrate_macrosreplace_names {
() => {
// Module: crate::macros
// Provides: {"replace_names"}
// Dependencies: {}
fn replace_names (input : & str) -> Option < (String , HashMap < String , String >) > { let mut result = String :: with_capacity (input . len () + 64) ; let mut substs = HashMap :: new () ; let mut dollar_count = 0 ; let mut cur_name = String :: new () ; for (kind , c) in CharClasses :: new (input . chars ()) { if kind != FullCodeCharKind :: Normal { result . push (c) ; } else if c == '$' { dollar_count += 1 ; } else if dollar_count == 0 { result . push (c) ; } else if ! c . is_alphanumeric () && ! cur_name . is_empty () { register_metavariable (& mut substs , & mut result , & cur_name , dollar_count) ; result . push (c) ; dollar_count = 0 ; cur_name . clear () ; } else if c == '(' && cur_name . is_empty () { return None ; } else if c . is_alphanumeric () || c == '_' { cur_name . push (c) ; } } if ! cur_name . is_empty () { register_metavariable (& mut substs , & mut result , & cur_name , dollar_count) ; } debug ! ("replace_names `{}` {:?}" , result , substs) ; Some ((result , substs)) }
};
}
