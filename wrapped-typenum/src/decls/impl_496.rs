macro_rules! deps {
    () => {
        UInt!();
        UTerm!();
        B1!();
        PrivateSquareRoot!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl PrivateSquareRoot for UInt < UTerm , B1 > { type Output = UInt < UTerm , B1 > ; }
    };
}

impl_496!()