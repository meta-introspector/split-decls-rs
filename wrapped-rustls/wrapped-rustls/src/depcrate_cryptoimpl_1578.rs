// Generated macro for impl_1578 (impl)
macro_rules! Depcrate_cryptoimpl_1578 {
() => {
// Module: crate::crypto
// Provides: {"impl_1578"}
// Dependencies: {}
impl SharedSecret { # [doc = " Returns the shared secret as a slice of bytes."] pub fn secret_bytes (& self) -> & [u8] { & self . buf [self . offset ..] } # [doc = " Removes leading zeros from `secret_bytes()` by adjusting the `offset`."] # [doc = ""] # [doc = " This function does not re-allocate."] fn strip_leading_zeros (& mut self) { let start = self . secret_bytes () . iter () . enumerate () . find (| (_i , x) | * * x != 0) . map (| (i , _x) | i) . unwrap_or_else (| | self . secret_bytes () . len ()) ; self . offset += start ; } }
};
}
