// Generated macro for impl_27 (impl)
macro_rules! Depcrate_entryimpl_27 {
() => {
// Module: crate::entry
// Provides: {"impl_27"}
// Dependencies: {}
impl ItemFn { # [doc = " Access all attributes of the function item."] fn attrs (& self) -> impl Iterator < Item = & Attribute > { self . outer_attrs . iter () . chain (self . inner_attrs . iter ()) } # [doc = " Get the body of the function item in a manner so that it can be"] # [doc = " conveniently used with the `quote!` macro."] fn body (& self) -> Body < '_ > { Body { brace_token : self . brace_token , stmts : & self . stmts , } } # [doc = " Convert our local function item into a token stream."] fn into_tokens (self , generated_attrs : proc_macro2 :: TokenStream , body : proc_macro2 :: TokenStream , last_block : proc_macro2 :: TokenStream ,) -> TokenStream { let mut tokens = proc_macro2 :: TokenStream :: new () ; for attr in self . outer_attrs { attr . to_tokens (& mut tokens) ; } for mut attr in self . inner_attrs { attr . style = syn :: AttrStyle :: Outer ; attr . to_tokens (& mut tokens) ; } generated_attrs . to_tokens (& mut tokens) ; self . vis . to_tokens (& mut tokens) ; self . sig . to_tokens (& mut tokens) ; self . brace_token . surround (& mut tokens , | tokens | { body . to_tokens (tokens) ; last_block . to_tokens (tokens) ; }) ; tokens } }
};
}
