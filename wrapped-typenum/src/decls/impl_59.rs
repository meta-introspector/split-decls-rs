macro_rules! deps {
    () => {
        Integer!();
        Z0!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [doc = " `Z0 + I = I`"] impl < I : Integer > Add < I > for Z0 { type Output = I ; # [inline] fn add (self , rhs : I) -> Self :: Output { rhs } }
    };
}

impl_59!();