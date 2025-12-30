// Generated macro for impl_722 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_722 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_722"}
// Dependencies: {}
impl < 'c , 'i , Side : SideData > ReadTraffic < 'c , 'i , Side > { fn new (conn : & 'c mut UnbufferedConnectionCommon < Side > , _incoming_tls : & 'i mut [u8]) -> Self { Self { conn , _incoming_tls , chunk : None , } } # [doc = " Decrypts and returns the next available app-data record"] pub fn next_record (& mut self) -> Option < Result < AppDataRecord < '_ > , Error > > { self . chunk = self . conn . core . common_state . received_plaintext . pop () ; self . chunk . as_ref () . map (| chunk | { Ok (AppDataRecord { discard : 0 , payload : chunk , }) }) } # [doc = " Returns the payload size of the next app-data record *without* decrypting it"] # [doc = ""] # [doc = " Returns `None` if there are no more app-data records"] pub fn peek_len (& self) -> Option < NonZeroUsize > { self . conn . core . common_state . received_plaintext . peek () . and_then (| ch | NonZeroUsize :: new (ch . len ())) } }
};
}
