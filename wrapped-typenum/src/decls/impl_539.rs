macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl Add < ATerm > for ATerm { type Output = ATerm ; # [inline] fn add (self , _ : ATerm) -> Self :: Output { ATerm } }
    };
}

impl_539!();