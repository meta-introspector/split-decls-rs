macro_rules! deps {
    () => {
        Result!();
        Pid!();
    };
}

macro_rules! impl_1660 {
    () => {
        deps!();
        # [cfg (lower_upper_exp_for_non_zero)] impl fmt :: UpperExp for Pid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1660!();