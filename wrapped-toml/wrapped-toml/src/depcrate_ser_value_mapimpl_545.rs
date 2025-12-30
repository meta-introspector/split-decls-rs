// Generated macro for impl_545 (impl)
macro_rules! Depcrate_ser_value_mapimpl_545 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'd > SerializeStructVariant < 'd > { pub (crate) fn struct_ (dst : & 'd mut String , variant : & 'static str , _len : usize , style : Style ,) -> Result < Self , Error > { dst . open_inline_table () ? ; dst . space () ? ; dst . key (variant) ? ; dst . space () ? ; dst . keyval_sep () ? ; dst . space () ? ; Ok (Self { inner : SerializeTable :: map (dst , style) ? , }) } }
};
}
