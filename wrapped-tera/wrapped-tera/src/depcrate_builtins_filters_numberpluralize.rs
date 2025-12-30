// Generated macro for pluralize (function)
macro_rules! Depcrate_builtins_filters_numberpluralize {
() => {
// Module: crate::builtins::filters::number
// Provides: {"pluralize"}
// Dependencies: {}
# [doc = " Returns a plural suffix if the value is not equal to ±1, or a singular"] # [doc = " suffix otherwise. The plural suffix defaults to `s` and the singular suffix"] # [doc = " defaults to the empty string (i.e nothing)."] pub fn pluralize (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let num = try_get_value ! ("pluralize" , "value" , f64 , value) ; let plural = match args . get ("plural") { Some (val) => try_get_value ! ("pluralize" , "plural" , String , val) , None => "s" . to_string () , } ; let singular = match args . get ("singular") { Some (val) => try_get_value ! ("pluralize" , "singular" , String , val) , None => "" . to_string () , } ; if (num . abs () - 1.) . abs () > :: std :: f64 :: EPSILON { Ok (to_value (plural) . unwrap ()) } else { Ok (to_value (singular) . unwrap ()) } }
};
}
