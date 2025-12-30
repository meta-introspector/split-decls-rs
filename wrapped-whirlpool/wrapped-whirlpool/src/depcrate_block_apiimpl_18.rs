// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl SerializableState for WhirlpoolCore { type SerializedStateSize = U72 ; fn serialize (& self) -> SerializedState < Self > { let mut res = Array :: < _ , U72 > :: default () ; let (state_dst , blocks_len_dst) = res . split_at_mut (64) ; for (val , chunk) in self . state . iter () . zip (state_dst . chunks_exact_mut (8)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } blocks_len_dst . copy_from_slice (& self . blocks_len . to_le_bytes ()) ; res } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (state_src , blocks_len_src) = serialized_state . split_at (64) ; let mut state = [0 ; STATE_LEN] ; for (val , chunk) in state . iter_mut () . zip (state_src . chunks_exact (8)) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let blocks_len = u64 :: from_le_bytes (blocks_len_src . try_into () . unwrap ()) ; Ok (Self { state , blocks_len }) } }
};
}
