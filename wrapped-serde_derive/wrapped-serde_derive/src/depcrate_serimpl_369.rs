// Generated macro for impl_369 (impl)
macro_rules! Depcrate_serimpl_369 {
() => {
// Module: crate::ser
// Provides: {"impl_369"}
// Dependencies: {}
impl StructTrait { fn serialize_field (& self , span : Span) -> TokenStream { match * self { StructTrait :: SerializeMap => { quote_spanned ! (span => _serde :: ser :: SerializeMap :: serialize_entry) } StructTrait :: SerializeStruct => { quote_spanned ! (span => _serde :: ser :: SerializeStruct :: serialize_field) } StructTrait :: SerializeStructVariant => { quote_spanned ! (span => _serde :: ser :: SerializeStructVariant :: serialize_field) } } } fn skip_field (& self , span : Span) -> Option < TokenStream > { match * self { StructTrait :: SerializeMap => None , StructTrait :: SerializeStruct => { Some (quote_spanned ! (span => _serde :: ser :: SerializeStruct :: skip_field)) } StructTrait :: SerializeStructVariant => { Some (quote_spanned ! (span => _serde :: ser :: SerializeStructVariant :: skip_field)) } } } }
};
}
