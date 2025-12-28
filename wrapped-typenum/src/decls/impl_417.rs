macro_rules! deps {
    () => {
        UTerm!();
        B1!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        # [doc = " `UTerm * B1 = UTerm`"] impl Mul < B1 > for UTerm { type Output = UTerm ; # [inline] fn mul (self , _ : B1) -> Self :: Output { UTerm } }
    };
}

impl_417!();