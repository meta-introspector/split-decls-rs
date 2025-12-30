// Generated macro for impl_28 (impl)
macro_rules! Depcrate_cowimpl_28 {
() => {
// Module: crate::cow
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized + Ord > Ord for VarZeroCow < 'a , V > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . deref () . cmp (other . deref ()) } }
};
}
