macro_rules! deps {
    () => {
        B0!();
        UTerm!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        # [doc = " `UTerm - B0 = Term`"] impl Sub < B0 > for UTerm { type Output = UTerm ; # [inline] fn sub (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_367!()