// Generated macro for macro_496 (macro)
macro_rules! Depcrate_itemmacro_496 {
() => {
// Module: crate::item
// Provides: {"macro_496"}
// Dependencies: {}
ast_struct ! { # [doc = " An impl block providing trait or associated items: `impl<A> Trait"] # [doc = " for Data<A> { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemImpl { pub attrs : Vec < Attribute >, pub defaultness : Option < Token ! [default] >, pub unsafety : Option < Token ! [unsafe] >, pub impl_token : Token ! [impl] , pub generics : Generics , # [doc = " Trait this impl implements."] pub trait_ : Option < (Option < Token ! [!] >, Path , Token ! [for]) >, # [doc = " The Self type of the impl."] pub self_ty : Box < Type >, pub brace_token : token :: Brace , pub items : Vec < ImplItem >, } }
};
}
