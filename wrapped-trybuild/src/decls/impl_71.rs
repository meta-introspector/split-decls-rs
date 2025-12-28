macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Error { pub fn already_printed (& self) -> bool { use self :: Error :: * ; matches ! (self , CargoFail | Mismatch | RunFailed | ShouldNotHaveCompiled) } }
    };
}

impl_71!();