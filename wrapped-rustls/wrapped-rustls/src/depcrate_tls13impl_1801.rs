// Generated macro for impl_1801 (impl)
macro_rules! Depcrate_tls13impl_1801 {
() => {
// Module: crate::tls13
// Provides: {"impl_1801"}
// Dependencies: {}
impl VerifyMessage { fn new (handshake_hash : & hash :: Output , context_string_with_0 : & [u8 ; 34]) -> Self { let used = 64 + context_string_with_0 . len () + handshake_hash . as_ref () . len () ; let mut buf = [0x20u8 ; MAX_VERIFY_MSG] ; let (_spaces , context) = buf . split_at_mut (64) ; let (context , hash) = context . split_at_mut (34) ; context . copy_from_slice (context_string_with_0) ; hash [.. handshake_hash . as_ref () . len ()] . copy_from_slice (handshake_hash . as_ref ()) ; Self { buf , used } } }
};
}
