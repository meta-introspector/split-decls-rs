// Generated macro for split (function)
macro_rules! Depcrate_builtins_filters_stringsplit {
() => {
// Module: crate::builtins::filters::string
// Provides: {"split"}
// Dependencies: {}
# [doc = " Split the given string by the given pattern."] pub fn split (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("split" , "value" , String , value) ; let pat = match args . get ("pat") { Some (pat) => { let p = try_get_value ! ("split" , "pat" , String , pat) ; p . replace ("\\n" , "\n") . replace ("\\t" , "\t") } None => return Err (Error :: msg ("Filter `split` expected an arg called `pat`")) , } ; Ok (to_value (s . split (& pat) . collect :: < Vec < _ > > ()) . unwrap ()) }
};
}
