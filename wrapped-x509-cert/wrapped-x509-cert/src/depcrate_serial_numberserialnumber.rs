// Generated macro for SerialNumber (struct)
macro_rules! Depcrate_serial_numberSerialNumber {
() => {
// Module: crate::serial_number
// Provides: {"SerialNumber"}
// Dependencies: {}
# [doc = " [RFC 5280 Section 4.1.2.2.]  Serial Number"] # [doc = ""] # [doc = "   The serial number MUST be a positive integer assigned by the CA to"] # [doc = "   each certificate.  It MUST be unique for each certificate issued by a"] # [doc = "   given CA (i.e., the issuer name and serial number identify a unique"] # [doc = "   certificate).  CAs MUST force the serialNumber to be a non-negative"] # [doc = "   integer."] # [doc = ""] # [doc = "   Given the uniqueness requirements above, serial numbers can be"] # [doc = "   expected to contain long integers.  Certificate users MUST be able to"] # [doc = "   handle serialNumber values up to 20 octets.  Conforming CAs MUST NOT"] # [doc = "   use serialNumber values longer than 20 octets."] # [doc = ""] # [doc = "   Note: Non-conforming CAs may issue certificates with serial numbers"] # [doc = "   that are negative or zero.  Certificate users SHOULD be prepared to"] # [doc = "   gracefully handle such certificates."] # [derive (Clone , Debug , Eq , PartialEq , ValueOrd , PartialOrd , Ord)] pub struct SerialNumber < P : Profile = Rfc5280 > { pub (crate) inner : Int , _profile : PhantomData < P > , }
};
}
