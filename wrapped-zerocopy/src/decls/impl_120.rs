macro_rules! deps {
    () => {
        ByteOrder!();
        BigEndian!();
        Order!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl ByteOrder for BigEndian { const ORDER : Order = Order :: BigEndian ; }
    };
}

impl_120!();