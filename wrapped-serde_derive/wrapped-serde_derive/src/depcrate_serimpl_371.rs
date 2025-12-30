// Generated macro for impl_371 (impl)
macro_rules! Depcrate_serimpl_371 {
() => {
// Module: crate::ser
// Provides: {"impl_371"}
// Dependencies: {}
impl TupleTrait { fn serialize_element (& self , span : Span) -> TokenStream { match * self { TupleTrait :: SerializeTuple => { quote_spanned ! (span => _serde :: ser :: SerializeTuple :: serialize_element) } TupleTrait :: SerializeTupleStruct => { quote_spanned ! (span => _serde :: ser :: SerializeTupleStruct :: serialize_field) } TupleTrait :: SerializeTupleVariant => { quote_spanned ! (span => _serde :: ser :: SerializeTupleVariant :: serialize_field) } } } }
};
}
