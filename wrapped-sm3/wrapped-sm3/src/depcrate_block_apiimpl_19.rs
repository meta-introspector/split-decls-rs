// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
impl SerializableState for Sm3Core { type SerializedStateSize = U40 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_h = SerializedState :: < Self > :: default () ; for (val , chunk) in self . h . iter () . zip (serialized_h . chunks_exact_mut (4)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_h [32 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_h } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_h , serialized_block_len) = serialized_state . split :: < U32 > () ; let mut h = [0 ; H_LEN] ; for (val , chunk) in h . iter_mut () . zip (serialized_h . chunks_exact (4)) { * val = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u64 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { block_len , h }) } }
};
}
