macro_rules! deps {
    () => {
        Error!();
        Result!();
        Metadata!();
    };
}

macro_rules! metadata {
    () => {
        deps!();
        pub (crate) fn metadata () -> Result < Metadata > { let output = raw_cargo () . arg ("metadata") . arg ("--no-deps") . arg ("--format-version=1") . output () . map_err (Error :: Cargo) ? ; serde_json :: from_slice (& output . stdout) . map_err (| err | { print ! ("{}" , String :: from_utf8_lossy (& output . stderr)) ; Error :: Metadata (err) }) }
    };
}

metadata!()