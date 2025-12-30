// Generated macro for impl_33 (impl)
macro_rules! Depcrate_msgs_baseimpl_33 {
() => {
// Module: crate::msgs::base
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > Codec < 'a > for SubjectPublicKeyInfoDer < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let nest = LengthPrefixedBuffer :: new (Self :: SIZE_LEN , bytes) ; nest . buf . extend (self . as_ref ()) ; } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let len = Self :: SIZE_LEN . read (r) ? ; let mut sub = r . sub (len) ? ; let body = sub . rest () ; Ok (Self :: from (body)) } }
};
}
