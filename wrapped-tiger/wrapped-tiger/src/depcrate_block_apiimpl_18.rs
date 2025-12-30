// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl < const V2 : bool > SerializableState for TigerCore < V2 > { type SerializedStateSize = U32 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; for (val , chunk) in self . state . iter () . zip (serialized_state . chunks_exact_mut (8)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state [24 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_state } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_state , serialized_block_len) = serialized_state . split :: < U24 > () ; let mut state = [0 ; STATE_LEN] ; for (val , chunk) in state . iter_mut () . zip (serialized_state . chunks_exact (8)) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u64 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { state , block_len }) } }
};
}
