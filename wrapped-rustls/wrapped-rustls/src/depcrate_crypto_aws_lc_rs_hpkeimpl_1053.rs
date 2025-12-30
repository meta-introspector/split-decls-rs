// Generated macro for impl_1053 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1053 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1053"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > HpkeAwsLcRs < KEY_SIZE , KDF_SIZE > { # [doc = " See [RFC 9180 §5.1 \"Creating the Encryption Context\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-5.1"] fn key_schedule (& self , shared_secret : KemSharedSecret < KDF_SIZE > , info : & [u8] ,) -> Result < KeySchedule < KEY_SIZE > , Error > { let suite_id = LabeledSuiteId :: Hpke (self . suite) ; let psk_id_hash = labeled_extract_for_prk (self . hkdf , suite_id , None , Label :: PskIdHash , & []) ; let info_hash = labeled_extract_for_prk (self . hkdf , suite_id , None , Label :: InfoHash , info) ; let key_schedule_context = [& [0] [..] , & psk_id_hash , & info_hash ,] . concat () ; let key = AeadKey (self . key_schedule_labeled_expand :: < KEY_SIZE > (& shared_secret , & key_schedule_context , Label :: Key ,)) ; let base_nonce = self . key_schedule_labeled_expand :: < NONCE_LEN > (& shared_secret , & key_schedule_context , Label :: BaseNonce ,) ; Ok (KeySchedule { aead : self . aead , key , base_nonce , seq_num : 0 , }) } fn key_schedule_labeled_expand < const L : usize > (& self , shared_secret : & KemSharedSecret < KDF_SIZE > , key_schedule_context : & [u8] , label : Label ,) -> [u8 ; L] { let suite_id = LabeledSuiteId :: Hpke (self . suite) ; labeled_expand :: < L > (suite_id , labeled_extract_for_expand (self . hkdf , suite_id , Some (& shared_secret . 0) , Label :: Secret , & [] ,) , label , key_schedule_context ,) } }
};
}
