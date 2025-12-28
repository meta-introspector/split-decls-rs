macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Mode { pub (crate) fn initialize (& self) -> Result < () , std :: io :: Error > { match self { Self :: Fail => { } Self :: Overwrite => { } Self :: Dump (root) => { std :: fs :: create_dir_all (root) ? ; let gitignore_path = root . join (".gitignore") ; std :: fs :: write (gitignore_path , "*\n") ? ; } } Ok (()) } }
    };
}

impl_79!()