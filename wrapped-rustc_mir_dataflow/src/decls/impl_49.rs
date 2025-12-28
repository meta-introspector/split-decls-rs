macro_rules! deps {
    () => {
        Formatter!();
        DebugWithContext!();
        MaybeReachable!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < S , C > DebugWithContext < C > for MaybeReachable < S > where S : DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MaybeReachable :: Unreachable => { write ! (f , "unreachable") } MaybeReachable :: Reachable (set) => set . fmt_with (ctxt , f) , } } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match (self , old) { (MaybeReachable :: Unreachable , MaybeReachable :: Unreachable) => Ok (()) , (MaybeReachable :: Unreachable , MaybeReachable :: Reachable (set)) => { write ! (f , "\u{001f}+") ? ; set . fmt_with (ctxt , f) } (MaybeReachable :: Reachable (set) , MaybeReachable :: Unreachable) => { write ! (f , "\u{001f}-") ? ; set . fmt_with (ctxt , f) } (MaybeReachable :: Reachable (this) , MaybeReachable :: Reachable (old)) => { this . fmt_diff_with (old , ctxt , f) } } } }
    };
}

impl_49!();