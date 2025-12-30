// Generated macro for read_func (function)
macro_rules! Depcrate_secure_transportread_func {
() => {
// Module: crate::secure_transport
// Provides: {"read_func"}
// Dependencies: {}
unsafe extern "C" fn read_func < S > (connection : SSLConnectionRef , data : * mut c_void , data_length : * mut usize ,) -> OSStatus where S : Read { let conn : & mut Connection < S > = & mut * (connection as * mut _) ; let mut read = 0 ; let ret = panic :: catch_unwind (AssertUnwindSafe (| | { let mut data = slice :: from_raw_parts_mut (data . cast :: < u8 > () , * data_length) ; while ! data . is_empty () { match conn . stream . read (data) { Ok (0) => return errSSLClosedNoNotify , Ok (len) => { let Some (rest) = data . get_mut (len ..) else { return errSecIO ; } ; data = rest ; read += len ; } , Err (e) => { let ret = translate_err (& e) ; conn . err = Some (e) ; return ret ; } , } } errSecSuccess })) . unwrap_or_else (| e | { conn . panic = Some (e) ; errSecIO }) ; * data_length = read ; ret }
};
}
