macro_rules! deps {
    () => {
        Z0!();
        PInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [doc = " `PInt + Z0 = PInt`"] impl < U : Unsigned + NonZero > Add < Z0 > for PInt < U > { type Output = PInt < U > ; # [inline] fn add (self , _ : Z0) -> Self :: Output { PInt :: new () } }
    };
}

impl_60!()