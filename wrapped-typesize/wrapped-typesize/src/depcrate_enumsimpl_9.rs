// Generated macro for impl_9 (impl)
macro_rules! Depcrate_enumsimpl_9 {
() => {
// Module: crate::enums
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for Option < T > { fn extra_size (& self) -> usize { self . as_ref () . map (T :: extra_size) . unwrap_or_default () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { self . as_ref () . and_then (T :: get_collection_item_count) } }
};
}
