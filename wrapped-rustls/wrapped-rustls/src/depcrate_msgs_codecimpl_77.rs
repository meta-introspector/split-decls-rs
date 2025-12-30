// Generated macro for impl_77 (impl)
macro_rules! Depcrate_msgs_codecimpl_77 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_77"}
// Dependencies: {}
impl Codec < '_ > for u24 { fn encode (& self , bytes : & mut Vec < u8 >) { let be_bytes = u32 :: to_be_bytes (self . 0) ; bytes . extend_from_slice (& be_bytes [1 ..]) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match r . take (3) { Some (& [a , b , c]) => Ok (Self (u32 :: from_be_bytes ([0 , a , b , c]))) , _ => Err (InvalidMessage :: MissingData ("u24")) , } } }
};
}
