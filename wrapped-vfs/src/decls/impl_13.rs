macro_rules! deps {
    () => {
        PrefixOf!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fst :: Automaton for PrefixOf < '_ > { type State = usize ; fn start (& self) -> usize { 0 } fn is_match (& self , & state : & usize) -> bool { state != ! 0 } fn can_match (& self , & state : & usize) -> bool { state != ! 0 } fn accept (& self , & state : & usize , byte : u8) -> usize { if self . prefix_of . get (state) == Some (& byte) { state + 1 } else { ! 0 } } }
    };
}

impl_13!();