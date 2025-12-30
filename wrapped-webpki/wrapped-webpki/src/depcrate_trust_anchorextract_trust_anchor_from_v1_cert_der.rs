// Generated macro for extract_trust_anchor_from_v1_cert_der (function)
macro_rules! Depcrate_trust_anchorextract_trust_anchor_from_v1_cert_der {
() => {
// Module: crate::trust_anchor
// Provides: {"extract_trust_anchor_from_v1_cert_der"}
// Dependencies: {}
# [doc = " Parses a v1 certificate directly into a TrustAnchor."] fn extract_trust_anchor_from_v1_cert_der (cert_der : untrusted :: Input < '_ > ,) -> Result < TrustAnchor < '_ > , Error > { cert_der . read_all (Error :: BadDer , | cert_der | { der :: nested (cert_der , der :: Tag :: Sequence , Error :: TrailingData (DerTypeId :: TrustAnchorV1) , | cert_der | { let anchor = der :: nested (cert_der , der :: Tag :: Sequence , Error :: TrailingData (DerTypeId :: TrustAnchorV1TbsCertificate) , | tbs | { lenient_certificate_serial_number (tbs) ? ; skip (tbs , der :: Tag :: Sequence) ? ; skip (tbs , der :: Tag :: Sequence) ? ; skip (tbs , der :: Tag :: Sequence) ? ; let subject = der :: expect_tag (tbs , der :: Tag :: Sequence) ? ; let spki = der :: expect_tag (tbs , der :: Tag :: Sequence) ? ; Ok (TrustAnchor { subject : subject . as_slice_less_safe () . into () , subject_public_key_info : spki . as_slice_less_safe () . into () , name_constraints : None , }) } ,) ; skip (cert_der , der :: Tag :: Sequence) ? ; skip (cert_der , der :: Tag :: BitString) ? ; anchor } ,) }) }
};
}
