macro_rules! deps {
    () => {
        Reference!();
        Type!();
        Region!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < R , T > fmt :: Display for Reference < R , T > where R : Region , T : Type , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("&") ? ; if self . is_mut { f . write_str ("mut ") ? ; } self . referent . fmt (f) } }
    };
}

impl_27!();