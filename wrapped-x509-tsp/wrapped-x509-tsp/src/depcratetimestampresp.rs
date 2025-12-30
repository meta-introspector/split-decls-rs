// Generated macro for TimeStampResp (struct)
macro_rules! DepcrateTimeStampResp {
() => {
// Module: crate
// Provides: {"TimeStampResp"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " TimeStampResp ::= SEQUENCE  {"] # [doc = "     status                  PKIStatusInfo,"] # [doc = "     timeStampToken          TimeStampToken     OPTIONAL  }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] pub struct TimeStampResp < 'a > { pub status : PkiStatusInfo < 'a > , # [asn1 (optional = "true")] pub time_stamp_token : Option < TimeStampToken > , }
};
}
