macro_rules! deps {
    () => {
        B0!();
        UTerm!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        # [doc = " `UTerm + B0 = UTerm`"] impl Add < B0 > for UTerm { type Output = UTerm ; # [inline] fn add (self , _ : B0) -> Self :: Output { UTerm } }
    };
}

impl_356!()