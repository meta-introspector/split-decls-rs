macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [cfg (feature = "borsh")] # [cfg_attr (docs_rs , doc (cfg (feature = "borsh")))] impl < A : Array > borsh :: BorshSerialize for TinyVec < A > where < A as Array > :: Item : borsh :: BorshSerialize , { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W ,) -> borsh :: io :: Result < () > { < usize as borsh :: BorshSerialize > :: serialize (& self . len () , writer) ? ; for elem in self . iter () { < < A as Array > :: Item as borsh :: BorshSerialize > :: serialize (elem , writer) ? ; } Ok (()) } }
    };
}

impl_129!();