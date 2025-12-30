// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_de_implstuple_impls {
() => {
// Module: crate::de::impls
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ len : tt => ($ ($ n : tt $ name : ident) +)) +) => { $ (# [cfg_attr (docsrs , doc (hidden))] impl <'de , $ ($ name) ,+> Deserialize <'de > for ($ ($ name ,) +) where $ ($ name : Deserialize <'de >,) + { tuple_impl_body ! ($ len => ($ ($ n $ name) +)) ; }) + } ; }
};
}
