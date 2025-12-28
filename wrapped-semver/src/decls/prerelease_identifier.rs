macro_rules! deps {
    () => {
        Error!();
        Prerelease!();
        Identifier!();
        Position!();
    };
}

macro_rules! prerelease_identifier {
    () => {
        deps!();
        fn prerelease_identifier (input : & str) -> Result < (Prerelease , & str) , Error > { let (string , rest) = identifier (input , Position :: Pre) ? ; let identifier = unsafe { Identifier :: new_unchecked (string) } ; Ok ((Prerelease { identifier } , rest)) }
    };
}

prerelease_identifier!()