// Generated macro for KeySchedule (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeKeySchedule {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"KeySchedule"}
// Dependencies: {}
# [doc = " KeySchedule holds the derived AEAD key, base nonce, and seq number"] # [doc = " common to both a [Sealer] and [Opener]."] struct KeySchedule < const KEY_SIZE : usize > { aead : & 'static aead :: Algorithm , key : AeadKey < KEY_SIZE > , base_nonce : [u8 ; NONCE_LEN] , seq_num : u32 , }
};
}
