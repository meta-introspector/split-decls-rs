macro_rules! deps {
    () => {
        DirFixture!();
        Error!();
        Result!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl DirFixture for String { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
    };
}

impl_173!()