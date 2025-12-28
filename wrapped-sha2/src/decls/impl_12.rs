macro_rules! deps {
    () => {
        Sha256VarCore!();
        State256!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl SerializableState for Sha256VarCore { type SerializedStateSize = U40 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; for (val , chunk) in self . state . iter () . zip (serialized_state . chunks_exact_mut (4)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state [32 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_state } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_state , serialized_block_len) = serialized_state . split :: < U32 > () ; let mut state = consts :: State256 :: default () ; for (val , chunk) in state . iter_mut () . zip (serialized_state . chunks_exact (4)) { * val = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u64 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { state , block_len }) } }
    };
}

impl_12!();