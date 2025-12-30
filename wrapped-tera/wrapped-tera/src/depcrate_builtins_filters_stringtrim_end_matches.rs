// Generated macro for trim_end_matches (function)
macro_rules! Depcrate_builtins_filters_stringtrim_end_matches {
() => {
// Module: crate::builtins::filters::string
// Provides: {"trim_end_matches"}
// Dependencies: {}
# [doc = " Strip trailing characters that match the given pattern."] pub fn trim_end_matches (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("trim_end_matches" , "value" , String , value) ; let pat = match args . get ("pat") { Some (pat) => { let p = try_get_value ! ("trim_end_matches" , "pat" , String , pat) ; p . replace ("\\n" , "\n") . replace ("\\t" , "\t") } None => return Err (Error :: msg ("Filter `trim_end_matches` expected an arg called `pat`")) , } ; Ok (to_value (s . trim_end_matches (& pat)) . unwrap ()) }
};
}
