// Generated macro for impl_796 (impl)
macro_rules! Depcrate_parsing_parsedimpl_796 {
() => {
// Module: crate::parsing::parsed
// Provides: {"impl_796"}
// Dependencies: {}
impl TryFrom < Parsed > for PrimitiveDateTime { type Error = error :: TryFromParsed ; # [inline] fn try_from (parsed : Parsed) -> Result < Self , Self :: Error > { Ok (Self :: new (parsed . try_into () ? , parsed . try_into () ?)) } }
};
}
