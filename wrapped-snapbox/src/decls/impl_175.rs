macro_rules! deps {
    () => {
        DirFixture!();
        Error!();
        Result!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < const N : usize , P , S > DirFixture for [(P , S) ; N] where P : AsRef < std :: path :: Path > , P : std :: fmt :: Debug , S : AsRef < [u8] > , S : std :: fmt :: Debug , { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { let s : & [(P , S)] = self ; s . write_to_path (root) } }
    };
}

impl_175!()