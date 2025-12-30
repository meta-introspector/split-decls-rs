// Generated macro for impl_175 (impl)
macro_rules! Depcrate_vecimpl_175 {
() => {
// Module: crate::vec
// Provides: {"impl_175"}
// Dependencies: {}
impl TypeSize for String { fn extra_size (& self) -> usize { core :: mem :: size_of :: < u8 > () * self . capacity () } # [cfg (feature = "details")] fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } }
};
}
