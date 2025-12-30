// Generated macro for transfer_eof (function)
macro_rules! Depcratetransfer_eof {
() => {
// Module: crate
// Provides: {"transfer_eof"}
// Dependencies: {}
pub fn transfer_eof (conn : & mut impl DerefMut < Target = ConnectionCommon < impl SideData > >) { let empty_buf = [0u8 ; 0] ; let empty_cursor : & mut dyn io :: Read = & mut & empty_buf [..] ; let sz = conn . read_tls (empty_cursor) . unwrap () ; assert_eq ! (sz , 0) ; }
};
}
