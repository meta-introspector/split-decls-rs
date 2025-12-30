// Generated macro for impl_161 (impl)
macro_rules! Depcrate_tendrilimpl_161 {
() => {
// Module: crate::tendril
// Provides: {"impl_161"}
// Dependencies: {}
impl < A > str :: FromStr for Tendril < fmt :: UTF8 , A > where A : Atomicity , { type Err = () ; # [inline] fn from_str (s : & str) -> Result < Self , () > { Ok (Tendril :: from_slice (s)) } }
};
}
