macro_rules! deps {
    () => {
        B0!();
        B1!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [doc = " Not of 0 (!0 = 1)"] impl Not for B0 { type Output = B1 ; # [inline] fn not (self) -> Self :: Output { B1 } }
    };
}

impl_9!()