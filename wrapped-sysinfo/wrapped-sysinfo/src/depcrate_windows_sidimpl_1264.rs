// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_windows_sidimpl_1264 {
() => {
// Module: crate::windows::sid
// Provides: {"impl_1264"}
// Dependencies: {}
impl Display for Sid { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { unsafe fn convert_sid_to_string_sid (sid : PSID) -> Option < String > { let mut string_sid = PWSTR :: null () ; unsafe { if let Err (_err) = ConvertSidToStringSidW (sid , & mut string_sid) { sysinfo_debug ! ("ConvertSidToStringSidW failed: {:?}" , _err) ; return None ; } let result = to_utf8_str (string_sid) ; let _err = LocalFree (Some (HLOCAL (string_sid . 0 as _))) ; Some (result) } } let string_sid = unsafe { convert_sid_to_string_sid (PSID ((self . sid . as_ptr () as * mut u8) . cast ())) } ; let string_sid = string_sid . ok_or (std :: fmt :: Error) ? ; write ! (f , "{string_sid}") } }
};
}
