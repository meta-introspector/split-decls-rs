// Generated macro for impl_1282 (impl)
macro_rules! Depcrate_validate_requestimpl_1282 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1282"}
// Dependencies: {}
impl < B , ResBody > ValidateRequest < B > for AcceptHeader < ResBody > where ResBody : Default , { type ResponseBody = ResBody ; fn validate (& mut self , req : & mut Request < B >) -> Result < () , Response < Self :: ResponseBody > > { if ! req . headers () . contains_key (header :: ACCEPT) { return Ok (()) ; } if req . headers () . get_all (header :: ACCEPT) . into_iter () . filter_map (| header | header . to_str () . ok ()) . any (| h | { MimeIter :: new (h) . map (| mim | { if let Ok (mim) = mim { let typ = self . header_value . type_ () ; let subtype = self . header_value . subtype () ; match (mim . type_ () , mim . subtype ()) { (t , s) if t == typ && s == subtype => true , (t , mime :: STAR) if t == typ => true , (mime :: STAR , mime :: STAR) => true , _ => false , } } else { false } }) . reduce (| acc , mim | acc || mim) . unwrap_or (false) }) { return Ok (()) ; } let mut res = Response :: new (ResBody :: default ()) ; * res . status_mut () = StatusCode :: NOT_ACCEPTABLE ; Err (res) } }
};
}
