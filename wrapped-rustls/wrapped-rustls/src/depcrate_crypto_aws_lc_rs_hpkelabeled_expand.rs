// Generated macro for labeled_expand (function)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkelabeled_expand {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"labeled_expand"}
// Dependencies: {}
# [doc = " See [RFC 9180 §4 \"Cryptographic Dependencies\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-4"] fn labeled_expand < const L : usize > (suite_id : LabeledSuiteId , expander : Box < dyn HkdfExpander > , label : Label , kem_context : & [u8] ,) -> [u8 ; L] { let output_len = u16 :: to_be_bytes (L as u16) ; let info = & [& output_len [..] , b"HPKE-v1" , & suite_id . encoded () , label . as_ref () , kem_context ,] ; expand (& * expander , info) }
};
}
