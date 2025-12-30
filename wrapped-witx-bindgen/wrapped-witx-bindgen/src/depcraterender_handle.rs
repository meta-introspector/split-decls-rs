// Generated macro for render_handle (function)
macro_rules! Depcraterender_handle {
() => {
// Module: crate
// Provides: {"render_handle"}
// Dependencies: {}
fn render_handle (src : & mut String , name : & str , _h : & HandleDatatype) { src . push_str (& format ! ("pub type {} = u32;" , name . to_camel_case ())) ; }
};
}
