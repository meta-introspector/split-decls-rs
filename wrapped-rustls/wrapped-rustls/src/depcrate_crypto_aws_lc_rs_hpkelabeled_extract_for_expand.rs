// Generated macro for labeled_extract_for_expand (function)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkelabeled_extract_for_expand {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"labeled_extract_for_expand"}
// Dependencies: {}
# [doc = " See [RFC 9180 §4 \"Cryptographic Dependencies\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-4"] fn labeled_extract_for_expand (hkdf : & 'static dyn HkdfPrkExtract , suite_id : LabeledSuiteId , salt : Option < & [u8] > , label : Label , ikm : & [u8] ,) -> Box < dyn HkdfExpander > { let labeled_ikm = [& b"HPKE-v1" [..] , & suite_id . encoded () , label . as_ref () , ikm] . concat () ; hkdf . extract_from_secret (salt , & labeled_ikm) }
};
}
