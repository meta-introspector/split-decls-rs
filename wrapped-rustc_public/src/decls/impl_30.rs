macro_rules! deps {
    () => {
        MachineInfo!();
        Scalar!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Scalar { pub fn has_niche (& self , target : & MachineInfo) -> bool { match self { Scalar :: Initialized { value , valid_range } => { ! valid_range . is_full (value . size (target)) . unwrap () } Scalar :: Union { .. } => false , } } }
    };
}

impl_30!();