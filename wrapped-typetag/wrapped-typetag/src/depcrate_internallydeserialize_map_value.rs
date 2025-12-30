// Generated macro for deserialize_map_value (macro)
macro_rules! Depcrate_internallydeserialize_map_value {
() => {
// Module: crate::internally
// Provides: {"deserialize_map_value"}
// Dependencies: {}
macro_rules ! deserialize_map_value { ($ self : ident , $ method : ident , $ visitor : ident $ (, $ k : ident : $ ty : ty) *) => { { struct Wrap < V > { $ ($ k : $ ty ,) * visitor : V , } impl <'de , V > DeserializeSeed <'de > for Wrap < V > where V : Visitor <'de >, { type Value = V :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer <'de >, { deserializer .$ method ($ (self .$ k ,) * self . visitor) } } let mut this = $ self ; this . map . next_value_seed (Wrap { $ ($ k ,) * $ visitor }) } } ; }
};
}
