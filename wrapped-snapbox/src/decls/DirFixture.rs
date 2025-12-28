macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! DirFixture {
    () => {
        deps!();
        # [doc = " Collection of files"] pub trait DirFixture : std :: fmt :: Debug { # [doc = " Initialize a test fixture directory `root`"] fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > ; }
    };
}

DirFixture!()