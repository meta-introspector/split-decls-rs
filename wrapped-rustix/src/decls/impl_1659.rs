macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! impl_1659 {
    () => {
        deps!();
        # [cfg (lower_upper_exp_for_non_zero)] impl fmt :: LowerExp for Pid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1659!()