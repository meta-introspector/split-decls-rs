macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl core :: fmt :: Display for Table { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { crate :: ser :: to_string (self) . expect ("Unable to represent value as string") . fmt (f) } }
    };
}

impl_373!()