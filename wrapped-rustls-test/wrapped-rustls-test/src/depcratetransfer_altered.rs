// Generated macro for transfer_altered (function)
macro_rules! Depcratetransfer_altered {
() => {
// Module: crate
// Provides: {"transfer_altered"}
// Dependencies: {}
pub fn transfer_altered < F > (left : & mut Connection , filter : F , right : & mut Connection) -> usize where F : Fn (& mut Message < '_ >) -> Altered , { let mut buf = [0u8 ; 262144] ; let mut total = 0 ; while left . wants_write () { let sz = { let into_buf : & mut dyn io :: Write = & mut & mut buf [..] ; left . write_tls (into_buf) . unwrap () } ; total += sz ; if sz == 0 { return total ; } let mut reader = Reader :: init (& buf [.. sz]) ; while reader . any_left () { let plain = PlainMessage :: read (& mut reader) . unwrap () ; let message_enc = match Message :: try_from (plain . clone ()) { Ok (mut message) => match filter (& mut message) { Altered :: InPlace => PlainMessage :: from (message) . into_unencrypted_opaque () . encode () , Altered :: Raw (data) => data , } , Err (_) => plain . into_unencrypted_opaque () . encode () , } ; let message_enc_reader : & mut dyn io :: Read = & mut & message_enc [..] ; let len = right . read_tls (message_enc_reader) . unwrap () ; assert_eq ! (len , message_enc . len ()) ; } } total }
};
}
