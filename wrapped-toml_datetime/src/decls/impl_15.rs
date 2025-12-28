macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl fmt :: Display for Time { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:02}:{:02}:{:02}" , self . hour , self . minute , self . second) ? ; if self . nanosecond != 0 { let s = alloc :: format ! ("{:09}" , self . nanosecond) ; write ! (f , ".{}" , s . trim_end_matches ('0')) ? ; } Ok (()) } }
    };
}

impl_15!();