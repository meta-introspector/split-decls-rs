macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl Sub < ATerm > for ATerm { type Output = ATerm ; # [inline] fn sub (self , _ : ATerm) -> Self :: Output { ATerm } }
    };
}

impl_541!()