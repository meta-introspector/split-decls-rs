// Generated macro for tuple_impls (macro)
macro_rules! Depcrate_tuplabletuple_impls {
() => {
// Module: crate::tuplable
// Provides: {"tuple_impls"}
// Dependencies: {}
macro_rules ! tuple_impls { ($ ($ len : expr => ($ ($ n : tt $ name : ident) +)) +) => { $ (impl <$ ($ name) ,+> Valuable for ($ ($ name ,) +) where $ ($ name : Valuable ,) + { fn as_value (& self) -> Value <'_ > { Value :: Tuplable (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_unnamed_fields (& [$ (self .$ n . as_value () ,) +]) ; } } impl <$ ($ name) ,+> Tuplable for ($ ($ name ,) +) where $ ($ name : Valuable ,) + { fn definition (& self) -> TupleDef { TupleDef :: Static { fields : $ len } } }) + } }
};
}
