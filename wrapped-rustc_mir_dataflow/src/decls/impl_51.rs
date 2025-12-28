macro_rules! deps {
    () => {
        DebugWithContext!();
        Formatter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T , C > DebugWithContext < C > for & '_ T where T : DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self) . fmt_with (ctxt , f) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self) . fmt_diff_with (* old , ctxt , f) } }
    };
}

impl_51!()