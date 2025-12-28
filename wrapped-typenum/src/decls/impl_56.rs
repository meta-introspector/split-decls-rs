macro_rules! deps {
    () => {
        Z0!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " `-Z0 = Z0`"] impl Neg for Z0 { type Output = Z0 ; # [inline] fn neg (self) -> Self :: Output { Z0 } }
    };
}

impl_56!()