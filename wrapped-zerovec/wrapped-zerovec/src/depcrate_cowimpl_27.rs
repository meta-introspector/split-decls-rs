// Generated macro for impl_27 (impl)
macro_rules! Depcrate_cowimpl_27 {
() => {
// Module: crate::cow
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized + PartialOrd > PartialOrd for VarZeroCow < 'a , V > { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . deref () . partial_cmp (other . deref ()) } }
};
}
