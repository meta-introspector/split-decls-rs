// Generated macro for impl_6 (impl)
macro_rules! Depcrate_cellimpl_6 {
() => {
// Module: crate::cell
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for core :: cell :: RefCell < T > { fn extra_size (& self) -> usize { self . borrow () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . borrow () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . borrow () . get_size_details () } } }
};
}
