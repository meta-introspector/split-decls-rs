macro_rules! deps {
    () => {
        UTerm!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        # [doc = " `UTerm - UTerm = UTerm`"] impl Sub < UTerm > for UTerm { type Output = UTerm ; # [inline] fn sub (self , _ : UTerm) -> Self :: Output { UTerm } }
    };
}

impl_372!();