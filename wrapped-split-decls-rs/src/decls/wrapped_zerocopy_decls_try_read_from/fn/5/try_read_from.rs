use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " # Safety"] # [doc = ""] # [doc = " All bytes of `candidate` must be initialized."] # [inline (always)] unsafe fn try_read_from < S , T : TryFromBytes > (source : S , mut candidate : CoreMaybeUninit < T > ,) -> Result < T , TryReadError < S , T > > { let c_ptr = Ptr :: from_mut (& mut candidate) ; let c_ptr = unsafe { c_ptr . assume_validity :: < invariant :: Initialized > () } ; let c_ptr = c_ptr . transmute () ; if ! Wrapping :: < T > :: is_bit_valid (c_ptr . forget_aligned ()) { return Err (ValidityError :: new (source) . into ()) ; } fn _assert_same_size_and_validity < T > () where Wrapping < T > : pointer :: TransmuteFrom < T , invariant :: Valid , invariant :: Valid > , T : pointer :: TransmuteFrom < Wrapping < T > , invariant :: Valid , invariant :: Valid > , { } _assert_same_size_and_validity :: < T > () ; Ok (unsafe { candidate . assume_init () }) }
}