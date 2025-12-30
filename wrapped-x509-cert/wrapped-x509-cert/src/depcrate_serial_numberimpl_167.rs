// Generated macro for impl_167 (impl)
macro_rules! Depcrate_serial_numberimpl_167 {
() => {
// Module: crate::serial_number
// Provides: {"impl_167"}
// Dependencies: {}
impl < P : Profile > SerialNumber < P > { # [doc = " Maximum length in bytes for a [`SerialNumber`]"] pub const MAX_LEN : Length = Length :: new (20) ; # [doc = " See notes in `SerialNumber::new` and `SerialNumber::decode_value`."] pub (crate) const MAX_DECODE_LEN : Length = Length :: new (21) ; # [doc = " Create a new [`SerialNumber`] from a byte slice."] # [doc = ""] # [doc = " The byte slice **must** represent a positive integer."] pub fn new (bytes : & [u8]) -> Result < Self > { let inner = asn1 :: Uint :: new (bytes) ? ; if inner . value_len () ? > Self :: MAX_LEN { return Err (ErrorKind :: Overlength . into ()) ; } Ok (Self { inner : inner . into () , _profile : PhantomData , }) } # [doc = " Borrow the inner byte slice which contains the least significant bytes"] # [doc = " of a big endian integer value with all leading zeros stripped."] pub fn as_bytes (& self) -> & [u8] { self . inner . as_bytes () } }
};
}
