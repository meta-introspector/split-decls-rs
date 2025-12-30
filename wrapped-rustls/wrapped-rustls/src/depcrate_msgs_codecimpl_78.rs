// Generated macro for impl_78 (impl)
macro_rules! Depcrate_msgs_codecimpl_78 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_78"}
// Dependencies: {}
impl Codec < '_ > for u32 { fn encode (& self , bytes : & mut Vec < u8 >) { bytes . extend (Self :: to_be_bytes (* self)) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match r . take (4) { Some (& [a , b , c , d]) => Ok (Self :: from_be_bytes ([a , b , c , d])) , _ => Err (InvalidMessage :: MissingData ("u32")) , } } }
};
}
