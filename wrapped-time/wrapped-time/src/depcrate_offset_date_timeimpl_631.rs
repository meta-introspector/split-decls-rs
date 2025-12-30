// Generated macro for impl_631 (impl)
macro_rules! Depcrate_offset_date_timeimpl_631 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_631"}
// Dependencies: {}
impl Hash for OffsetDateTime { # [inline] fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { raw_to_bits (self . to_utc_raw ()) . hash (state) ; } }
};
}
