// Generated macro for impl_127 (impl)
macro_rules! Depcrate_ptr_arcimpl_127 {
() => {
// Module: crate::ptr::arc
// Provides: {"impl_127"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for SizableArc < T , Owned > { fn extra_size (& self) -> usize { T :: get_size (& self . 0) + (core :: mem :: size_of :: < AtomicUsize > () * 2) } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { T :: get_collection_item_count (& self . 0) } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { T :: get_size_details (& self . 0) } } }
};
}
