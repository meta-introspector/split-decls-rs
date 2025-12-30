// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for SignerError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { SignerError :: KeypairPubkeyMismatch => f . write_str ("keypair-pubkey mismatch") , SignerError :: NotEnoughSigners => f . write_str ("not enough signers") , SignerError :: TransactionError (_) => f . write_str ("transaction error") , SignerError :: Custom (e) => write ! (f , "custom error: {e}" ,) , SignerError :: PresignerError (_) => f . write_str ("presigner error") , SignerError :: Connection (e) => write ! (f , "connection error: {e}" ,) , SignerError :: InvalidInput (s) => write ! (f , "invalid input: {s}" ,) , SignerError :: NoDeviceFound => f . write_str ("no device found") , SignerError :: Protocol (s) => { write ! (f , "{s}") } SignerError :: UserCancel (s) => { write ! (f , "{s}") } SignerError :: TooManySigners => f . write_str ("too many signers") , } } }
};
}
