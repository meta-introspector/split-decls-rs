// Generated macro for TimeStampReq (struct)
macro_rules! DepcrateTimeStampReq {
() => {
// Module: crate
// Provides: {"TimeStampReq"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " TimeStampReq ::= SEQUENCE  {"] # [doc = "    version               INTEGER  { v1(1) },"] # [doc = "    messageImprint        MessageImprint,"] # [doc = "    reqPolicy             TSAPolicyId              OPTIONAL,"] # [doc = "    nonce                 INTEGER                  OPTIONAL,"] # [doc = "    certReq               BOOLEAN                  DEFAULT FALSE,"] # [doc = "    extensions            [0] IMPLICIT Extensions  OPTIONAL  }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] pub struct TimeStampReq { pub version : TspVersion , pub message_imprint : MessageImprint , # [asn1 (optional = "true")] pub req_policy : Option < TsaPolicyId > , # [asn1 (optional = "true")] pub nonce : Option < Int > , # [asn1 (default = "Default::default")] pub cert_req : bool , # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , optional = "true")] pub extensions : Option < Extensions > , }
};
}
