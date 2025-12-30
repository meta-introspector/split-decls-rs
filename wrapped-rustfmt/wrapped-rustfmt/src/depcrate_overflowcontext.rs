// Generated macro for Context (struct)
macro_rules! Depcrate_overflowContext {
() => {
// Module: crate::overflow
// Provides: {"Context"}
// Dependencies: {}
struct Context < 'a > { context : & 'a RewriteContext < 'a > , items : Vec < OverflowableItem < 'a > > , ident : & 'a str , prefix : & 'static str , suffix : & 'static str , one_line_shape : Shape , nested_shape : Shape , span : Span , item_max_width : usize , one_line_width : usize , force_separator_tactic : Option < SeparatorTactic > , custom_delims : Option < (& 'a str , & 'a str) > , }
};
}
