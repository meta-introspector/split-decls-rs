// Generated macro for render_record (function)
macro_rules! Depcraterender_record {
() => {
// Module: crate
// Provides: {"render_record"}
// Dependencies: {}
fn render_record (src : & mut String , name : & str , s : & RecordDatatype) { if let Some (repr) = s . bitflags_repr () { src . push_str (& format ! ("pub type {} = " , name . to_camel_case ())) ; repr . render (src) ; src . push (';') ; for (i , member) in s . members . iter () . enumerate () { rustdoc (& member . docs , src) ; src . push_str (& format ! ("pub const {}_{}: {} = 1 << {};\n" , name . to_shouty_snake_case () , member . name . as_str () . to_shouty_snake_case () , name . to_camel_case () , i ,)) ; } return ; } src . push_str ("#[repr(C)]\n") ; if record_contains_union (s) { src . push_str ("#[derive(Copy, Clone)]\n") ; } else { src . push_str ("#[derive(Copy, Clone, Debug)]\n") ; } src . push_str (& format ! ("pub struct {} {{\n" , name . to_camel_case ())) ; for member in s . members . iter () { rustdoc (& member . docs , src) ; src . push_str ("pub ") ; member . name . render (src) ; src . push_str (": ") ; member . tref . render (src) ; src . push_str (",\n") ; } src . push_str ("}") ; }
};
}
