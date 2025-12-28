macro_rules! deps {
    () => {
        Result!();
        Error!();
        DirFixture!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl DirFixture for & '_ str { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
    };
}

impl_171!();