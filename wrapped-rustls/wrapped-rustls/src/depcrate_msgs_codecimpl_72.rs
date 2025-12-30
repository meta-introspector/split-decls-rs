// Generated macro for impl_72 (impl)
macro_rules! Depcrate_msgs_codecimpl_72 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_72"}
// Dependencies: {}
impl Codec < '_ > for u8 { fn encode (& self , bytes : & mut Vec < u8 >) { bytes . push (* self) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match r . take (1) { Some (& [byte]) => Ok (byte) , _ => Err (InvalidMessage :: MissingData ("u8")) , } } }
};
}
