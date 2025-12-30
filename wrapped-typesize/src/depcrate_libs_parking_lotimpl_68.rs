// Generated macro for impl_68 (impl)
macro_rules! Depcrate_libs_parking_lotimpl_68 {
() => {
// Module: crate::libs::parking_lot
// Provides: {"impl_68"}
// Dependencies: {}
impl < T : TypeSize > TypeSize for parking_lot :: Mutex < T > { fn extra_size (& self) -> usize { self . lock () . extra_size () } if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { self . lock () . get_collection_item_count () } fn get_size_details (& self) -> alloc :: vec :: Vec < crate :: Field > { self . lock () . get_size_details () } } }
};
}
