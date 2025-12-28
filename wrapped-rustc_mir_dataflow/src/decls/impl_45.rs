macro_rules! deps {
    () => {
        DebugDiffWithAdapter!();
        DebugWithContext!();
        Formatter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for DebugDiffWithAdapter < '_ , T , C > where T : DebugWithContext < C > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . new . fmt_diff_with (& self . old , self . ctxt , f) } }
    };
}

impl_45!();