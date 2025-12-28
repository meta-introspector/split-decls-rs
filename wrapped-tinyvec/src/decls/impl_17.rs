macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [cfg (feature = "borsh")] # [cfg_attr (docs_rs , doc (cfg (feature = "borsh")))] impl < A : Array > borsh :: BorshDeserialize for ArrayVec < A > where < A as Array > :: Item : borsh :: BorshDeserialize , { fn deserialize_reader < R : borsh :: io :: Read > (reader : & mut R ,) -> borsh :: io :: Result < Self > { let len = < usize as borsh :: BorshDeserialize > :: deserialize_reader (reader) ? ; let mut new_arrayvec = Self :: default () ; for idx in 0 .. len { let value = < < A as Array > :: Item as borsh :: BorshDeserialize > :: deserialize_reader (reader ,) ? ; if idx >= new_arrayvec . capacity () { return Err (borsh :: io :: Error :: new (borsh :: io :: ErrorKind :: InvalidData , "invalid ArrayVec length" ,)) ; } new_arrayvec . push (value) } Ok (new_arrayvec) } }
    };
}

impl_17!();