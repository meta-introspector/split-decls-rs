// Generated macro for render_variant (function)
macro_rules! Depcraterender_variant {
() => {
// Module: crate
// Provides: {"render_variant"}
// Dependencies: {}
fn render_variant (src : & mut String , name : & str , v : & Variant) { if v . cases . iter () . all (| c | c . tref . is_none ()) { return render_enum_like_variant (src , name , v) ; } src . push_str ("#[repr(C)]\n") ; src . push_str ("#[derive(Copy, Clone)]\n") ; src . push_str (& format ! ("pub union {}U {{\n" , name . to_camel_case ())) ; for case in v . cases . iter () { if let Some (ref tref) = case . tref { rustdoc (& case . docs , src) ; src . push_str ("pub ") ; case . name . render (src) ; src . push_str (": ") ; tref . render (src) ; src . push_str (",\n") ; } } src . push_str ("}\n") ; src . push_str ("#[repr(C)]\n") ; src . push_str ("#[derive(Copy, Clone)]\n") ; src . push_str (& format ! ("pub struct {} {{\n" , name . to_camel_case ())) ; src . push_str ("pub tag: ") ; v . tag_repr . render (src) ; src . push_str (",\n") ; src . push_str (& format ! ("pub u: {}U,\n" , name . to_camel_case ())) ; src . push_str ("}\n") ; }
};
}
