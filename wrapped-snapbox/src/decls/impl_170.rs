macro_rules! deps {
    () => {
        Error!();
        DirFixture!();
        Result!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl DirFixture for str { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
    };
}

impl_170!();