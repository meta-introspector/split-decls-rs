// Generated macro for impl_510 (impl)
macro_rules! Depcrate_ser_value_arrayimpl_510 {
() => {
// Module: crate::ser::value::array
// Provides: {"impl_510"}
// Dependencies: {}
impl < 'd > SerializeTupleVariant < 'd > { pub (crate) fn tuple (dst : & 'd mut String , variant : & 'static str , len : usize , style : Style ,) -> Result < Self , Error > { dst . open_inline_table () ? ; dst . space () ? ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; Ok (Self { inner : SerializeValueArray :: seq (dst , style , Some (len)) ? , }) } }
};
}
