macro_rules! deps {
    () => {
        B1!();
        B0!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [doc = " Not of 1 (!1 = 0)"] impl Not for B1 { type Output = B0 ; # [inline] fn not (self) -> Self :: Output { B0 } }
    };
}

impl_10!()