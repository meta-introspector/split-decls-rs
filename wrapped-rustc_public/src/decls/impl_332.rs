macro_rules! deps {
    () => {
        IntTy!();
        MachineInfo!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl IntTy { pub fn num_bytes (self) -> usize { match self { IntTy :: Isize => MachineInfo :: target_pointer_width () . bytes () , IntTy :: I8 => 1 , IntTy :: I16 => 2 , IntTy :: I32 => 4 , IntTy :: I64 => 8 , IntTy :: I128 => 16 , } } }
    };
}

impl_332!();