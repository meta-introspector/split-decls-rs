// Generated macro for impl_80 (impl)
macro_rules! Depcrate_msgs_codecimpl_80 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_80"}
// Dependencies: {}
impl Codec < '_ > for u64 { fn encode (& self , bytes : & mut Vec < u8 >) { let mut b64 = [0u8 ; 8] ; put_u64 (* self , & mut b64) ; bytes . extend_from_slice (& b64) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match r . take (8) { Some (& [a , b , c , d , e , f , g , h]) => Ok (Self :: from_be_bytes ([a , b , c , d , e , f , g , h])) , _ => Err (InvalidMessage :: MissingData ("u64")) , } } }
};
}
