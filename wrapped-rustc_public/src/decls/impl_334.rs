macro_rules! deps {
    () => {
        MachineInfo!();
        UintTy!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl UintTy { pub fn num_bytes (self) -> usize { match self { UintTy :: Usize => MachineInfo :: target_pointer_width () . bytes () , UintTy :: U8 => 1 , UintTy :: U16 => 2 , UintTy :: U32 => 4 , UintTy :: U64 => 8 , UintTy :: U128 => 16 , } } }
    };
}

impl_334!();