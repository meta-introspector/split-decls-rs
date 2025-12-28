macro_rules! deps {
    () => {
        BigEndian!();
        ByteOrder!();
        Order!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl ByteOrder for BigEndian { const ORDER : Order = Order :: BigEndian ; }
    };
}

impl_120!()