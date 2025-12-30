// Generated macro for impl_393 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_393 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_393"}
// Dependencies: {}
impl Codec < '_ > for EchConfigPayload { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: V18 (c) => { EchVersion :: V18 . encode (bytes) ; let inner = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; c . encode (inner . buf) ; } Self :: Unknown { version , contents } => { version . encode (bytes) ; contents . encode (bytes) ; } } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let version = EchVersion :: read (r) ? ; let length = u16 :: read (r) ? ; let mut contents = r . sub (length as usize) ? ; Ok (match version { EchVersion :: V18 => Self :: V18 (EchConfigContents :: read (& mut contents) ?) , _ => { let data = PayloadU16 :: new (contents . rest () . into ()) ; Self :: Unknown { version , contents : data , } } }) } }
};
}
