// Generated macro for impl_62 (impl)
macro_rules! Depcrate_msgs_ccsimpl_62 {
() => {
// Module: crate::msgs::ccs
// Provides: {"impl_62"}
// Dependencies: {}
impl Codec < '_ > for ChangeCipherSpecPayload { fn encode (& self , bytes : & mut Vec < u8 >) { 1u8 . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let typ = u8 :: read (r) ? ; if typ != 1 { return Err (InvalidMessage :: InvalidCcs) ; } r . expect_empty ("ChangeCipherSpecPayload") . map (| _ | Self { }) } }
};
}
