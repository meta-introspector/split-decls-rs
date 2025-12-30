// Generated macro for proofs (module)
macro_rules! Depcrate_byte_sliceproofs {
() => {
// Module: crate::byte_slice
// Provides: {"proofs"}
// Dependencies: {}
# [cfg (kani)] mod proofs { use super :: * ; fn any_vec () -> Vec < u8 > { let len = kani :: any () ; kani :: assume (len <= isize :: MAX as usize) ; vec ! [0u8 ; len] } # [kani :: proof] fn prove_split_at_unchecked () { let v = any_vec () ; let slc = v . as_slice () ; let mid = kani :: any () ; kani :: assume (mid <= slc . len ()) ; let (l , r) = unsafe { slc . split_at_unchecked (mid) } ; assert_eq ! (l . len () + r . len () , slc . len ()) ; let slc : * const _ = slc ; let l : * const _ = l ; let r : * const _ = r ; assert_eq ! (slc . cast ::< u8 > () , l . cast ::< u8 > ()) ; assert_eq ! (unsafe { slc . cast ::< u8 > () . add (mid) } , r . cast ::< u8 > ()) ; let mut v = any_vec () ; let slc = v . as_mut_slice () ; let len = slc . len () ; let mid = kani :: any () ; kani :: assume (mid <= slc . len ()) ; let (l , r) = unsafe { slc . split_at_unchecked (mid) } ; assert_eq ! (l . len () + r . len () , len) ; let l : * mut _ = l ; let r : * mut _ = r ; let slc : * mut _ = slc ; assert_eq ! (slc . cast ::< u8 > () , l . cast ::< u8 > ()) ; assert_eq ! (unsafe { slc . cast ::< u8 > () . add (mid) } , r . cast ::< u8 > ()) ; } }
};
}
