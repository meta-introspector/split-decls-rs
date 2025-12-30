// Generated macro for impl_13 (impl)
macro_rules! Depcrate_spannedimpl_13 {
() => {
// Module: crate::spanned
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : core :: fmt :: Display > core :: fmt :: Display for Spanned < T > { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . get_ref () . fmt (fmt) } }
};
}
