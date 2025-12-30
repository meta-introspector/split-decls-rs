// Generated macro for macro_135 (macro)
macro_rules! Depcrate_itemmacro_135 {
() => {
// Module: crate::item
// Provides: {"macro_135"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " An argument in a function header."] # [doc = ""] # [doc = " E.g. `bar: usize` as in `fn foo(bar: usize)`"] pub enum FnArg { pub SelfRef (ArgSelfRef { pub and_token : tokens :: And , pub self_token : tokens :: Self_ , pub lifetime : Option < Lifetime >, pub mutbl : Mutability , }) , pub SelfValue (ArgSelf { pub mutbl : Mutability , pub self_token : tokens :: Self_ , }) , pub Captured (ArgCaptured { pub pat : Pat , pub colon_token : tokens :: Colon , pub ty : Ty , }) , pub Ignored (Ty) , } }
};
}
