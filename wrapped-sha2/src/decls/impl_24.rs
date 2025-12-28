macro_rules! deps {
    () => {
        Sha512VarCore!();
        State512!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl SerializableState for Sha512VarCore { type SerializedStateSize = U80 ; fn serialize (& self) -> SerializedState < Self > { let mut serialized_state = SerializedState :: < Self > :: default () ; for (val , chunk) in self . state . iter () . zip (serialized_state . chunks_exact_mut (8)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_state [64 ..] . copy_from_slice (& self . block_len . to_le_bytes ()) ; serialized_state } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_state , serialized_block_len) = serialized_state . split :: < U64 > () ; let mut state = consts :: State512 :: default () ; for (val , chunk) in state . iter_mut () . zip (serialized_state . chunks_exact (8)) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let block_len = u128 :: from_le_bytes (* serialized_block_len . as_ref ()) ; Ok (Self { state , block_len }) } }
    };
}

impl_24!()