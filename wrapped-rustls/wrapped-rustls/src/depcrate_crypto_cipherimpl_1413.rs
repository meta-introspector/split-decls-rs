// Generated macro for impl_1413 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1413 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1413"}
// Dependencies: {}
impl Iv { # [doc = " Create a new `Iv` from a byte slice."] # [doc = ""] # [doc = " Returns an error if the length of `value` exceeds [`Self::MAX_LEN`]."] pub fn new (value : & [u8]) -> Result < Self , Error > { if value . len () > Self :: MAX_LEN { return Err (ApiMisuse :: IvLengthExceedsMaximum { actual : value . len () , maximum : Self :: MAX_LEN , } . into ()) ; } let mut buf = [0u8 ; Self :: MAX_LEN] ; buf [.. value . len ()] . copy_from_slice (value) ; Ok (Self { buf , used : value . len () , }) } # [doc = " Return the IV length."] # [expect (clippy :: len_without_is_empty)] pub fn len (& self) -> usize { self . used } # [doc = " Maximum supported IV length."] pub const MAX_LEN : usize = 16 ; }
};
}
