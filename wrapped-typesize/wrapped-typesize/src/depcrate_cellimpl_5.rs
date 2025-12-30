// Generated macro for impl_5 (impl)
macro_rules! Depcrate_cellimpl_5 {
() => {
// Module: crate::cell
// Provides: {"impl_5"}
// Dependencies: {}
impl < T : TypeSize + Copy > TypeSize for core :: cell :: Cell < T > { fn extra_size (& self) -> usize { self . get () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . get () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . get () . get_size_details () } } }
};
}
