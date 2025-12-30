// Generated macro for impl_22 (impl)
macro_rules! Depcrate_block_apiimpl_22 {
() => {
// Module: crate::block_api
// Provides: {"impl_22"}
// Dependencies: {}
impl SerializableState for StreebogVarCore { type SerializedStateSize = U192 ; fn serialize (& self) -> SerializedState < Self > { let ser_h : Array < u8 , U64 > = to_bytes (& self . h) . into () ; let ser_n : Array < u8 , U64 > = to_bytes (& self . n) . into () ; let ser_sigma : Array < u8 , U64 > = to_bytes (& self . sigma) . into () ; ser_h . concat (ser_n) . concat (ser_sigma) } fn deserialize (ser_state : & SerializedState < Self >) -> Result < Self , DeserializeStateError > { let (ser_h , rem) = ser_state . split :: < U64 > () ; let (ser_n , ser_sigma) = rem . split :: < U64 > () ; Ok (Self { h : from_bytes (& ser_h . into ()) , n : from_bytes (& ser_n . into ()) , sigma : from_bytes (& ser_sigma . into ()) , }) } }
};
}
