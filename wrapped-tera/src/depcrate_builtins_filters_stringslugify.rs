// Generated macro for slugify (function)
macro_rules! Depcrate_builtins_filters_stringslugify {
() => {
// Module: crate::builtins::filters::string
// Provides: {"slugify"}
// Dependencies: {}
# [doc = " Transform a string into a slug"] # [cfg (feature = "builtins")] pub fn slugify (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("slugify" , "value" , String , value) ; Ok (to_value (slug :: slugify (s)) . unwrap ()) }
};
}
