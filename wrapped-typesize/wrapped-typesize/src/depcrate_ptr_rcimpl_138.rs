// Generated macro for impl_138 (impl)
macro_rules! Depcrate_ptr_rcimpl_138 {
() => {
// Module: crate::ptr::rc
// Provides: {"impl_138"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for SizableRc < T , Owned > { fn extra_size (& self) -> usize { T :: get_size (& self . 0) + (core :: mem :: size_of :: < Cell < usize > > () * 2) } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { T :: get_collection_item_count (& self . 0) } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { T :: get_size_details (& self . 0) } } }
};
}
