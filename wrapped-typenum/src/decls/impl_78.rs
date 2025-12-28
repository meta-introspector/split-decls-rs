macro_rules! deps {
    () => {
        Z0!();
        Integer!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        # [doc = " `Z0 * I = Z0`"] impl < I : Integer > Mul < I > for Z0 { type Output = Z0 ; # [inline] fn mul (self , _ : I) -> Self :: Output { Z0 } }
    };
}

impl_78!();