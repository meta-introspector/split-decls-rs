// Generated macro for impl_33 (impl)
macro_rules! Depcrate_stable_vecimpl_33 {
() => {
// Module: crate::stable_vec
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > From < Vec < T > > for StableVec < T > { fn from (other : Vec < T >) -> Self { let mut other = ManuallyDrop :: new (other) ; Self { addr : other . as_mut_ptr () as u64 , cap : other . capacity () as u64 , len : other . len () as u64 , _marker : PhantomData , } } }
};
}
