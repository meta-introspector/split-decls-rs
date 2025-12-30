// Generated macro for impl_69 (impl)
macro_rules! Depcrate_libs_parking_lotimpl_69 {
() => {
// Module: crate::libs::parking_lot
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for parking_lot :: RwLock < T > { fn extra_size (& self) -> usize { self . read () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . read () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . read () . get_size_details () } } }
};
}
