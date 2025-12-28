macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Handle { pub fn from_path < P : AsRef < Path > > (_p : P) -> io :: Result < Handle > { error () } pub fn from_file (_file : File) -> io :: Result < Handle > { error () } pub fn stdin () -> io :: Result < Handle > { error () } pub fn stdout () -> io :: Result < Handle > { error () } pub fn stderr () -> io :: Result < Handle > { error () } pub fn as_file (& self) -> & File { unreachable ! (ERROR_MESSAGE) ; } pub fn as_file_mut (& self) -> & mut File { unreachable ! (ERROR_MESSAGE) ; } }
    };
}

impl_14!();