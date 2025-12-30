// Generated macro for impl_410 (impl)
macro_rules! Depcrate_ser_document_arrayimpl_410 {
() => {
// Module: crate::ser::document::array
// Provides: {"impl_410"}
// Dependencies: {}
impl < 'd > SerializeDocumentTupleVariant < 'd > { pub (crate) fn tuple (buf : & 'd mut Buffer , mut table : Table , variant : & 'static str , _len : usize , style : Style ,) -> Result < Self , Error > { let dst = table . body_mut () ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; dst . open_array () ? ; Ok (Self { buf , table , seen_value : false , style , }) } }
};
}
