// Generated macro for transfer (function)
macro_rules! Depcratetransfer {
() => {
// Module: crate
// Provides: {"transfer"}
// Dependencies: {}
pub fn transfer (left : & mut impl DerefMut < Target = ConnectionCommon < impl SideData > > , right : & mut impl DerefMut < Target = ConnectionCommon < impl SideData > > ,) -> usize { let mut buf = [0u8 ; 262144] ; let mut total = 0 ; while left . wants_write () { let sz = { let into_buf : & mut dyn io :: Write = & mut & mut buf [..] ; left . write_tls (into_buf) . unwrap () } ; total += sz ; if sz == 0 { return total ; } let mut offs = 0 ; loop { let from_buf : & mut dyn io :: Read = & mut & buf [offs .. sz] ; offs += right . read_tls (from_buf) . unwrap () ; if sz == offs { break ; } } } total }
};
}
