// Generated macro for impl_157 (impl)
macro_rules! Depcrate_syncimpl_157 {
() => {
// Module: crate::sync
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for Mutex < T > { fn extra_size (& self) -> usize { self . lock () . unwrap_or_else (PoisonError :: into_inner) . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . lock () . unwrap_or_else (PoisonError :: into_inner) . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . lock () . unwrap_or_else (PoisonError :: into_inner) . get_size_details () } } }
};
}
