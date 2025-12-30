// Generated macro for P_REAL_MAX (const)
macro_rules! Depcrate_byte_phfP_REAL_MAX {
() => {
// Module: crate::byte_phf
// Provides: {"P_REAL_MAX"}
// Dependencies: {}
# [doc = " The maximum allowable value of `p`. This could be raised if found to be necessary."] # [doc = " Values exceeding P_FAST_MAX could use a different `p` algorithm by modifying [`f1`]."] # [cfg (feature = "alloc")] const P_REAL_MAX : u8 = P_FAST_MAX ;
};
}
