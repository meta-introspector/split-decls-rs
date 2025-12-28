macro_rules! deps {
    () => {
        B0!();
        UTerm!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        # [doc = " `UTerm * B0 = UTerm`"] impl Mul < B0 > for UTerm { type Output = UTerm ; # [inline] fn mul (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_416!()