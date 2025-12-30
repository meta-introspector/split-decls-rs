// Generated macro for OcspResponseStatus (enum)
macro_rules! Depcrate_responseOcspResponseStatus {
() => {
// Module: crate::response
// Provides: {"OcspResponseStatus"}
// Dependencies: {}
# [doc = " OCSPResponseStatus structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " OCSPResponseStatus ::= ENUMERATED {"] # [doc = "    successful          (0),  -- Response has valid confirmations"] # [doc = "    malformedRequest    (1),  -- Illegal confirmation request"] # [doc = "    internalError       (2),  -- Internal error in issuer"] # [doc = "    tryLater            (3),  -- Try again later"] # [doc = "                              -- (4) is not used"] # [doc = "    sigRequired         (5),  -- Must sign the request"] # [doc = "    unauthorized        (6)   -- Request unauthorized"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Enumerated , Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] # [allow (missing_docs)] pub enum OcspResponseStatus { Successful = 0 , MalformedRequest = 1 , InternalError = 2 , TryLater = 3 , SigRequired = 5 , Unauthorized = 6 , }
};
}
