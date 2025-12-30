// Generated macro for impl_1265 (impl)
macro_rules! Depcrate_windows_sidimpl_1265 {
() => {
// Module: crate::windows::sid
// Provides: {"impl_1265"}
// Dependencies: {}
impl FromStr for Sid { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { unsafe { let mut string_sid : Vec < u16 > = s . encode_utf16 () . collect () ; string_sid . push (0) ; let mut psid = PSID :: default () ; if let Err (err) = ConvertStringSidToSidW (PCWSTR :: from_raw (string_sid . as_ptr ()) , & mut psid) { return Err (format ! ("ConvertStringSidToSidW failed: {err:?}")) ; } let sid = Self :: from_psid (psid) ; let _err = LocalFree (Some (HLOCAL (psid . 0 as _))) ; Ok (sid . unwrap ()) } } }
};
}
