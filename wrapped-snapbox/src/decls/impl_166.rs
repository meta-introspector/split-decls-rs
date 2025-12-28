macro_rules! deps {
    () => {
        DirFixture!();
        Result!();
        Error!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl DirFixture for std :: path :: Path { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { super :: copy_template (self , root) } }
    };
}

impl_166!();