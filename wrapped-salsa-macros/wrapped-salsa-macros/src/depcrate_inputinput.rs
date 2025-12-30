// Generated macro for input (function)
macro_rules! Depcrate_inputinput {
() => {
// Module: crate::input
// Provides: {"input"}
// Dependencies: {}
# [doc = " For an entity struct `Foo` with fields `f1: T1, ..., fN: TN`, we generate..."] # [doc = ""] # [doc = " * the \"id struct\" `struct Foo(salsa::Id)`"] # [doc = " * the entity ingredient, which maps the id fields to the `Id`"] # [doc = " * for each value field, a function ingredient"] pub (crate) fn input (args : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let args = syn :: parse_macro_input ! (args as InputArgs) ; let hygiene = Hygiene :: from1 (& input) ; let struct_item = parse_macro_input ! (input as syn :: ItemStruct) ; let m = Macro { hygiene , args , struct_item , } ; match m . try_macro () { Ok (v) => v . into () , Err (e) => token_stream_with_error (input , e) , } }
};
}
