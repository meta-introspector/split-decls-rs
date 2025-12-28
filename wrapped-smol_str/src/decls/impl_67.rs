macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl BorshSerialize for SmolStr { fn serialize < W : Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { self . as_str () . serialize (writer) } }
    };
}

impl_67!();