macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl Neg for ATerm { type Output = ATerm ; # [inline] fn neg (self) -> Self :: Output { ATerm } }
    };
}

impl_557!()