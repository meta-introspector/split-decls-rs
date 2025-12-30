// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_ser_implstuple_impls {
() => {
// Module: crate::ser::impls
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ len : expr => ($ ($ n : tt $ name : ident) +)) +) => { $ (# [cfg_attr (docsrs , doc (hidden))] impl <$ ($ name) ,+> Serialize for ($ ($ name ,) +) where $ ($ name : Serialize ,) + { tuple_impl_body ! ($ len => ($ ($ n) +)) ; }) + } ; }
};
}
