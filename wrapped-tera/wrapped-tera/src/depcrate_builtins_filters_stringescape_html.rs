// Generated macro for escape_html (function)
macro_rules! Depcrate_builtins_filters_stringescape_html {
() => {
// Module: crate::builtins::filters::string
// Provides: {"escape_html"}
// Dependencies: {}
# [doc = " Returns the given text with all special HTML characters encoded"] pub fn escape_html (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("escape_html" , "value" , String , value) ; Ok (Value :: String (utils :: escape_html (& s))) }
};
}
