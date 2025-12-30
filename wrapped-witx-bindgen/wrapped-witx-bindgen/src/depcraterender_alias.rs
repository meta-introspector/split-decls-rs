// Generated macro for render_alias (function)
macro_rules! Depcraterender_alias {
() => {
// Module: crate
// Provides: {"render_alias"}
// Dependencies: {}
fn render_alias (src : & mut String , name : & str , dest : & TypeRef) { src . push_str (& format ! ("pub type {}" , name . to_camel_case ())) ; if let Type :: List (_) = & * * dest . type_ () { src . push_str ("<'a>") ; } src . push_str (" = ") ; if name == "size" { src . push_str ("usize") ; } else { dest . render (src) ; } src . push (';') ; }
};
}
