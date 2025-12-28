macro_rules! deps {
    () => {
        ArrayVecVisitor!();
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , A : Array > Visitor < 'de > for ArrayVecVisitor < A > where A :: Item : Deserialize < 'de > , { type Value = ArrayVec < A > ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < S > (self , mut seq : S) -> Result < Self :: Value , S :: Error > where S : SeqAccess < 'de > , { let mut new_arrayvec : ArrayVec < A > = Default :: default () ; let mut idx = 0usize ; while let Some (value) = seq . next_element () ? { if new_arrayvec . len () >= new_arrayvec . capacity () { return Err (DeserializeError :: invalid_length (idx , & self)) ; } new_arrayvec . push (value) ; idx = idx + 1 ; } Ok (new_arrayvec) } }
    };
}

impl_70!();