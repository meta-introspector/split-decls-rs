macro_rules! deps {
    () => {
        Identifier!();
        Position!();
        BuildMetadata!();
        Error!();
    };
}

macro_rules! build_identifier {
    () => {
        deps!();
        fn build_identifier (input : & str) -> Result < (BuildMetadata , & str) , Error > { let (string , rest) = identifier (input , Position :: Build) ? ; let identifier = unsafe { Identifier :: new_unchecked (string) } ; Ok ((BuildMetadata { identifier } , rest)) }
    };
}

build_identifier!()