macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl SerializableState for Sha1Core { type SerializedStateSize = U28 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_h = SerializedState :: < Self > :: default () ; for (val , chunk) in self . h . iter () . zip (serialized_h . chunks_exact_mut (4)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_h [20 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_h } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_h , serialized_block_len) = serialized_state . split :: < U20 > () ; let mut h = [0 ; STATE_LEN] ; for (val , chunk) in h . iter_mut () . zip (serialized_h . chunks_exact (4)) { * val = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u64 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { h , block_len }) } }
    };
}

impl_15!();