macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! parse_git_version {
    () => {
        deps!();
        fn parse_git_version () -> Result < (u8 , u8 , u8) > { let output = std :: process :: Command :: new (GIT_PROGRAM) . arg ("--version") . output () ? ; git_version_from_bytes (& output . stdout) }
    };
}

parse_git_version!()