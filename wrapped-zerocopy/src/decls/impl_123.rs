macro_rules! deps {
    () => {
        ByteOrder!();
        LittleEndian!();
        Order!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl ByteOrder for LittleEndian { const ORDER : Order = Order :: LittleEndian ; }
    };
}

impl_123!();