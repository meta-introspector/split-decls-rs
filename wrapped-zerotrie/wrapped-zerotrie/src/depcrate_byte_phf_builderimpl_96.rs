// Generated macro for impl_96 (impl)
macro_rules! Depcrate_byte_phf_builderimpl_96 {
() => {
// Module: crate::byte_phf::builder
// Provides: {"impl_96"}
// Dependencies: {}
impl PerfectByteHashMap < Vec < u8 > > { # [doc = " Computes a new [`PerfectByteHashMap`]."] # [doc = ""] # [doc = " (this is a doc-hidden API)"] # [allow (clippy :: indexing_slicing)] pub fn try_new (keys : & [u8]) -> Result < Self , ZeroTrieBuildError > { let n_usize = keys . len () ; let n = n_usize as u8 ; let (p , mut qq) = find (keys) ? ; let mut keys_permuted = vec ! [0 ; n_usize] ; for key in keys { let l1 = f1 (* key , p , n) as usize ; let q = qq [l1] ; let l2 = f2 (* key , q , n) as usize ; keys_permuted [l2] = * key ; } let mut result = Vec :: with_capacity (n_usize * 2 + 1) ; result . push (p) ; result . append (& mut qq) ; result . append (& mut keys_permuted) ; Ok (Self (result)) } }
};
}
