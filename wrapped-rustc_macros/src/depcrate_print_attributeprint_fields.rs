// Generated macro for print_fields (function)
macro_rules! Depcrate_print_attributeprint_fields {
() => {
// Module: crate::print_attribute
// Provides: {"print_fields"}
// Dependencies: {}
fn print_fields (name : & Ident , fields : & Fields) -> (TokenStream , TokenStream) { let string_name = name . to_string () ; let mut disps = vec ! [quote ! { let mut __printed_anything = false ; }] ; match fields { Fields :: Named (fields_named) => { let mut field_names = Vec :: new () ; for field in & fields_named . named { let name = field . ident . as_ref () . unwrap () ; let string_name = name . to_string () ; disps . push (quote ! { if # name . should_render () { if __printed_anything { __p . word_space (",") ; } __p . word (# string_name) ; __p . word (":") ; __p . nbsp () ; __printed_anything = true ; } # name . print_attribute (__p) ; }) ; field_names . push (name) ; } (quote ! { { # (# field_names) ,* } } , quote ! { __p . word (# string_name) ; if true # (&& !# field_names . should_render ()) * { return ; } __p . nbsp () ; __p . word ("{") ; # (# disps) * __p . word ("}") ; } ,) } Fields :: Unnamed (fields_unnamed) => { let mut field_names = Vec :: new () ; for idx in 0 .. fields_unnamed . unnamed . len () { let name = format_ident ! ("f{idx}") ; disps . push (quote ! { if # name . should_render () { if __printed_anything { __p . word_space (",") ; } __printed_anything = true ; } # name . print_attribute (__p) ; }) ; field_names . push (name) ; } (quote ! { (# (# field_names) ,*) } , quote ! { __p . word (# string_name) ; if true # (&& !# field_names . should_render ()) * { return ; } __p . popen () ; # (# disps) * __p . pclose () ; } ,) } Fields :: Unit => (quote ! { } , quote ! { __p . word (# string_name) }) , } }
};
}
