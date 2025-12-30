// Generated macro for deserialize_default_key (macro)
macro_rules! Depcrate_internallydeserialize_default_key {
() => {
// Module: crate::internally
// Provides: {"deserialize_default_key"}
// Dependencies: {}
macro_rules ! deserialize_default_key { ($ self : ident , $ method : ident , $ visitor : ident $ (, $ k : ident : $ ty : ty) *) => { { struct Wrap < V > { $ ($ k : $ ty ,) * visitor : V , } impl <'de , V > DeserializeSeed <'de > for Wrap < V > where V : Visitor <'de >, { type Value = V :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer <'de >, { deserializer .$ method ($ (self .$ k ,) * self . visitor) } } let mut this = $ self ; this . try_default_key () ?; this . map . next_value_seed (Wrap { $ ($ k ,) * $ visitor }) } } ; }
};
}
