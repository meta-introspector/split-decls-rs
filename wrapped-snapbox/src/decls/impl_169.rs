macro_rules! deps {
    () => {
        DirFixture!();
        Result!();
        Error!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl DirFixture for std :: path :: PathBuf { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
    };
}

impl_169!();