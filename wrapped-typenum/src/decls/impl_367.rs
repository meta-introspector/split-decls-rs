macro_rules! deps {
    () => {
        UTerm!();
        B0!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        # [doc = " `UTerm - B0 = Term`"] impl Sub < B0 > for UTerm { type Output = UTerm ; # [inline] fn sub (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_367!();