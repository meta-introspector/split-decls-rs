// Generated macro for ConnectionSecrets (struct)
macro_rules! Depcrate_tls12ConnectionSecrets {
() => {
// Module: crate::tls12
// Provides: {"ConnectionSecrets"}
// Dependencies: {}
# [doc = " TLS1.2 per-connection keying material"] pub (crate) struct ConnectionSecrets { pub (crate) randoms : ConnectionRandoms , suite : & 'static Tls12CipherSuite , master_secret : Zeroizing < [u8 ; 48] > , # [doc = " `master_secret` ready to be used as a TLS1.2 PRF secret."] # [doc = ""] # [doc = " Zeroizing this on drop is left to the implementer of the trait."] master_secret_prf : Box < dyn crypto :: tls12 :: PrfSecret > , }
};
}
