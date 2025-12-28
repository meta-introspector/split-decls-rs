macro_rules! deps {
    () => {
        Z0!();
        Pow!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [doc = " 0^0 = 1"] impl Pow < Z0 > for Z0 { type Output = P1 ; # [inline] fn powi (self , _ : Z0) -> Self :: Output { P1 :: new () } }
    };
}

impl_107!()