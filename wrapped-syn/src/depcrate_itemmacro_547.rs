// Generated macro for macro_547 (macro)
macro_rules! Depcrate_itemmacro_547 {
() => {
// Module: crate::item
// Provides: {"macro_547"}
// Dependencies: {}
ast_struct ! { # [doc = " The `self` argument of an associated method."] # [doc = ""] # [doc = " If `colon_token` is present, the receiver is written with an explicit"] # [doc = " type such as `self: Box<Self>`. If `colon_token` is absent, the receiver"] # [doc = " is written in shorthand such as `self` or `&self` or `&mut self`. In the"] # [doc = " shorthand case, the type in `ty` is reconstructed as one of `Self`,"] # [doc = " `&Self`, or `&mut Self`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Receiver { pub attrs : Vec < Attribute >, pub reference : Option < (Token ! [&] , Option < Lifetime >) >, pub mutability : Option < Token ! [mut] >, pub self_token : Token ! [self] , pub colon_token : Option < Token ! [:] >, pub ty : Box < Type >, } }
};
}
