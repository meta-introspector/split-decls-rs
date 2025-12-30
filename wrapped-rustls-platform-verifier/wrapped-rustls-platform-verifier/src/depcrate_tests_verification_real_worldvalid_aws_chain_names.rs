// Generated macro for valid_aws_chain_names (function)
macro_rules! Depcrate_tests_verification_real_worldvalid_aws_chain_names {
() => {
// Module: crate::tests::verification_real_world
// Provides: {"valid_aws_chain_names"}
// Dependencies: {}
# [doc = " Returns a list of names valid for [VALID_AWS_AMAZON_COM_CHAIN], in a format"] # [doc = " expected by `CertificateError::NotValidForContext`."] # [cfg (not (any (target_vendor = "apple" , windows)))] fn valid_aws_chain_names () -> Vec < String > { const VALID_AWS_NAMES : & [& str] = & ["aws.amazon.com" , "www.aws.amazon.com" , "aws-us-east-1.amazon.com" , "aws-us-west-2.amazon.com" , "amazonaws-china.com" , "www.amazonaws-china.com" , "1.aws-lbr.amazonaws.com" ,] ; VALID_AWS_NAMES . iter () . copied () . map (| name | format ! ("DnsName(\"{name}\")")) . collect () }
};
}
