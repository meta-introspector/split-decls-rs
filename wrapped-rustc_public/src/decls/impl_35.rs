macro_rules! deps {
    () => {
        IntegerLength!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl IntegerLength { pub fn bits (self) -> usize { match self { IntegerLength :: I8 => 8 , IntegerLength :: I16 => 16 , IntegerLength :: I32 => 32 , IntegerLength :: I64 => 64 , IntegerLength :: I128 => 128 , } } }
    };
}

impl_35!();