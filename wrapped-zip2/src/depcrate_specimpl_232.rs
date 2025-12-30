// Generated macro for impl_232 (impl)
macro_rules! Depcrate_specimpl_232 {
() => {
// Module: crate::spec
// Provides: {"impl_232"}
// Dependencies: {}
impl Magic { pub const fn literal (x : u32) -> Self { Self (x) } # [inline (always)] # [allow (dead_code)] pub const fn from_le_bytes (bytes : [u8 ; 4]) -> Self { Self (u32 :: from_le_bytes (bytes)) } # [inline (always)] pub const fn to_le_bytes (self) -> [u8 ; 4] { self . 0 . to_le_bytes () } # [allow (clippy :: wrong_self_convention)] # [inline (always)] pub fn from_le (self) -> Self { Self (u32 :: from_le (self . 0)) } # [allow (clippy :: wrong_self_convention)] # [inline (always)] pub fn to_le (self) -> Self { Self (u32 :: to_le (self . 0)) } pub const LOCAL_FILE_HEADER_SIGNATURE : Self = Self :: literal (0x04034b50) ; pub const CENTRAL_DIRECTORY_HEADER_SIGNATURE : Self = Self :: literal (0x02014b50) ; pub const CENTRAL_DIRECTORY_END_SIGNATURE : Self = Self :: literal (0x06054b50) ; pub const ZIP64_CENTRAL_DIRECTORY_END_SIGNATURE : Self = Self :: literal (0x06064b50) ; pub const ZIP64_CENTRAL_DIRECTORY_END_LOCATOR_SIGNATURE : Self = Self :: literal (0x07064b50) ; pub const DATA_DESCRIPTOR_SIGNATURE : Self = Self :: literal (0x08074b50) ; }
};
}
