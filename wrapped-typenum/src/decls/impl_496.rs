macro_rules! deps {
    () => {
        PrivateSquareRoot!();
        UTerm!();
        B1!();
        UInt!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl PrivateSquareRoot for UInt < UTerm , B1 > { type Output = UInt < UTerm , B1 > ; }
    };
}

impl_496!();