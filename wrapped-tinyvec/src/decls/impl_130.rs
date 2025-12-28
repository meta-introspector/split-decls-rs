macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (feature = "borsh")] # [cfg_attr (docs_rs , doc (cfg (feature = "borsh")))] impl < A : Array > borsh :: BorshDeserialize for TinyVec < A > where < A as Array > :: Item : borsh :: BorshDeserialize , { fn deserialize_reader < R : borsh :: io :: Read > (reader : & mut R ,) -> borsh :: io :: Result < Self > { let len = < usize as borsh :: BorshDeserialize > :: deserialize_reader (reader) ? ; let mut new_tinyvec = Self :: with_capacity (len) ; for _ in 0 .. len { new_tinyvec . push (< < A as Array > :: Item as borsh :: BorshDeserialize > :: deserialize_reader (reader ,) ? ,) } Ok (new_tinyvec) } }
    };
}

impl_130!();