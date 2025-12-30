// Generated macro for impl_724 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_724 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_724"}
// Dependencies: {}
impl < 'c , 'i > ReadEarlyData < 'c , 'i , ServerConnectionData > { fn new (conn : & 'c mut UnbufferedConnectionCommon < ServerConnectionData > , _incoming_tls : & 'i mut [u8] ,) -> Self { Self { conn , _incoming_tls , chunk : None , } } # [doc = " decrypts and returns the next available app-data record"] pub fn next_record (& mut self) -> Option < Result < AppDataRecord < '_ > , Error > > { self . chunk = self . conn . pop_early_data () ; self . chunk . as_ref () . map (| chunk | { Ok (AppDataRecord { discard : 0 , payload : chunk , }) }) } # [doc = " returns the payload size of the next app-data record *without* decrypting it"] # [doc = ""] # [doc = " returns `None` if there are no more app-data records"] pub fn peek_len (& self) -> Option < NonZeroUsize > { self . conn . peek_early_data () . and_then (| ch | NonZeroUsize :: new (ch . len ())) } }
};
}
