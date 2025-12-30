// Generated macro for write_func (function)
macro_rules! Depcrate_secure_transportwrite_func {
() => {
// Module: crate::secure_transport
// Provides: {"write_func"}
// Dependencies: {}
unsafe extern "C" fn write_func < S > (connection : SSLConnectionRef , data : * const c_void , data_length : * mut usize ,) -> OSStatus where S : Write { let conn : & mut Connection < S > = & mut * (connection as * mut _) ; let mut written = 0 ; let ret = panic :: catch_unwind (AssertUnwindSafe (| | { let mut data = slice :: from_raw_parts (data . cast :: < u8 > () , * data_length) ; while ! data . is_empty () { match conn . stream . write (data) { Ok (0) => return errSSLClosedNoNotify , Ok (len) => { let Some (rest) = data . get (len ..) else { return errSecIO ; } ; data = rest ; written += len ; } , Err (e) => { let ret = translate_err (& e) ; conn . err = Some (e) ; return ret ; } , } } if let Err (e) = conn . stream . flush () { let ret = translate_err (& e) ; conn . err = Some (e) ; return ret ; } errSecSuccess })) . unwrap_or_else (| e | { conn . panic = Some (e) ; errSecIO }) ; * data_length = written ; ret }
};
}
