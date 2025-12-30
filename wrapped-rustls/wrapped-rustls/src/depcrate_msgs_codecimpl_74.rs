// Generated macro for impl_74 (impl)
macro_rules! Depcrate_msgs_codecimpl_74 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_74"}
// Dependencies: {}
impl Codec < '_ > for u16 { fn encode (& self , bytes : & mut Vec < u8 >) { let mut b16 = [0u8 ; 2] ; put_u16 (* self , & mut b16) ; bytes . extend_from_slice (& b16) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match r . take (2) { Some (& [b1 , b2]) => Ok (Self :: from_be_bytes ([b1 , b2])) , _ => Err (InvalidMessage :: MissingData ("u16")) , } } }
};
}
