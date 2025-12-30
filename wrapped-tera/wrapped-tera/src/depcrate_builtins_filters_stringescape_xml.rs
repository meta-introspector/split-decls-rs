// Generated macro for escape_xml (function)
macro_rules! Depcrate_builtins_filters_stringescape_xml {
() => {
// Module: crate::builtins::filters::string
// Provides: {"escape_xml"}
// Dependencies: {}
# [doc = " Returns the given text with all special XML characters encoded"] # [doc = " Very similar to `escape_html`, just a few characters less are encoded"] pub fn escape_xml (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("escape_html" , "value" , String , value) ; let mut output = String :: with_capacity (s . len () * 2) ; for c in s . chars () { match c { '&' => output . push_str ("&amp;") , '<' => output . push_str ("&lt;") , '>' => output . push_str ("&gt;") , '"' => output . push_str ("&quot;") , '\'' => output . push_str ("&apos;") , _ => output . push (c) , } } Ok (Value :: String (output)) }
};
}
