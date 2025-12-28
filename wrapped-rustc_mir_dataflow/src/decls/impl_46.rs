macro_rules! deps {
    () => {
        DebugWithContext!();
        Formatter!();
        DebugWithAdapter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T , C > DebugWithContext < C > for DenseBitSet < T > where T : Idx + DebugWithContext < C > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter () . map (| i | DebugWithAdapter { this : i , ctxt })) . finish () } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let size = self . domain_size () ; assert_eq ! (size , old . domain_size ()) ; let mut set_in_self = MixedBitSet :: new_empty (size) ; let mut cleared_in_self = MixedBitSet :: new_empty (size) ; for i in (0 .. size) . map (T :: new) { match (self . contains (i) , old . contains (i)) { (true , false) => set_in_self . insert (i) , (false , true) => cleared_in_self . insert (i) , _ => continue , } ; } fmt_diff (& set_in_self , & cleared_in_self , ctxt , f) } }
    };
}

impl_46!();