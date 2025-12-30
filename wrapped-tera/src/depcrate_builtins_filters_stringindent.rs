// Generated macro for indent (function)
macro_rules! Depcrate_builtins_filters_stringindent {
() => {
// Module: crate::builtins::filters::string
// Provides: {"indent"}
// Dependencies: {}
# [doc = " Indents a string by the specified width."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `value`   - The string to indent."] # [doc = " * `args`    - A set of key/value arguments that can take the following"] # [doc = "   keys."] # [doc = " * `prefix`  - The prefix used for indentation. The default value is 4 spaces."] # [doc = " * `first`  - True indents the first line.  The default is false."] # [doc = " * `blank`  - True indents blank lines.  The default is false."] # [doc = ""] pub fn indent (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("indent" , "value" , String , value) ; let prefix = match args . get ("prefix") { Some (p) => try_get_value ! ("indent" , "prefix" , String , p) , None => "    " . to_string () , } ; let first = match args . get ("first") { Some (f) => try_get_value ! ("indent" , "first" , bool , f) , None => false , } ; let blank = match args . get ("blank") { Some (b) => try_get_value ! ("indent" , "blank" , bool , b) , None => false , } ; let mut out = String :: with_capacity (s . len () + (prefix . len () * (s . chars () . filter (| & c | c == '\n') . count () + 1)) ,) ; let mut first_pass = true ; for line in s . lines () { if first_pass { if first { out . push_str (& prefix) ; } first_pass = false ; } else { out . push ('\n') ; if blank || ! line . trim_start () . is_empty () { out . push_str (& prefix) ; } } out . push_str (line) ; } Ok (to_value (& out) . unwrap ()) }
};
}
