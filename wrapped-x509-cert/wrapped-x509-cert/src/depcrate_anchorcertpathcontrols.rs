// Generated macro for CertPathControls (struct)
macro_rules! Depcrate_anchorCertPathControls {
() => {
// Module: crate::anchor
// Provides: {"CertPathControls"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " CertPathControls ::= SEQUENCE {"] # [doc = "     taName              Name,"] # [doc = "     certificate         [0] Certificate OPTIONAL,"] # [doc = "     policySet           [1] CertificatePolicies OPTIONAL,"] # [doc = "     policyFlags         [2] CertPolicyFlags OPTIONAL,"] # [doc = "     nameConstr          [3] NameConstraints OPTIONAL,"] # [doc = "     pathLenConstraint   [4] INTEGER (0..MAX) OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CertPathControls < P : Profile = Rfc5280 > { pub ta_name : Name , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub certificate : Option < CertificateInner < P > > , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , optional = "true")] pub policy_set : Option < CertificatePolicies > , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT" , optional = "true")] pub policy_flags : Option < CertPolicyFlags > , # [asn1 (context_specific = "3" , tag_mode = "IMPLICIT" , optional = "true")] pub name_constr : Option < NameConstraints > , # [asn1 (context_specific = "4" , tag_mode = "IMPLICIT" , optional = "true")] pub path_len_constraint : Option < u32 > , }
};
}
