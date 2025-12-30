// Generated macro for enum_try_as (function)
macro_rules! Depcrateenum_try_as {
() => {
// Module: crate
// Provides: {"enum_try_as"}
// Dependencies: {}
# [doc = " Generated `try_as_*()` methods for all tuple-style variants."] # [doc = " E.g. `Message.try_as_write()`."] # [doc = ""] # [doc = " These methods will only be generated for tuple-style variants, not for named or unit variants."] # [doc = ""] # [doc = " ```"] # [doc = " use strum_macros::EnumTryAs;"] # [doc = ""] # [doc = " #[derive(EnumTryAs, Debug)]"] # [doc = " enum Message {"] # [doc = "     Quit,"] # [doc = "     Move { x: i32, y: i32 },"] # [doc = "     Write(String),"] # [doc = "     ChangeColor(i32, i32, i32),"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     Message::Write(String::from(\"Hello\")).try_as_write(),"] # [doc = "     Some(String::from(\"Hello\"))"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     Message::ChangeColor(1, 2, 3).try_as_change_color(),"] # [doc = "     Some((1, 2, 3))"] # [doc = " );"] # [doc = " ```"] # [proc_macro_derive (EnumTryAs , attributes (strum))] pub fn enum_try_as (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_try_as :: enum_try_as_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
