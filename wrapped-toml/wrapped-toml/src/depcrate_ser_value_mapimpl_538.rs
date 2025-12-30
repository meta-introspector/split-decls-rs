// Generated macro for impl_538 (impl)
macro_rules! Depcrate_ser_value_mapimpl_538 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_538"}
// Dependencies: {}
impl < 'd > SerializeTable < 'd > { pub (crate) fn map (dst : & 'd mut String , style : Style) -> Result < Self , Error > { dst . open_inline_table () ? ; Ok (Self { dst , seen_value : false , key : None , style , }) } pub (crate) fn end (self) -> Result < & 'd mut String , Error > { if self . seen_value { self . dst . space () ? ; } self . dst . close_inline_table () ? ; Ok (self . dst) } }
};
}
