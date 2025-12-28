macro_rules! impl_472 {
    () => {
        impl FromStr for LinkSelfContainedComponents { type Err = String ; # [doc = " Parses a single `-Clink-self-contained` well-known component, not a set of flags."] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "crto" => LinkSelfContainedComponents :: CRT_OBJECTS , "libc" => LinkSelfContainedComponents :: LIBC , "unwind" => LinkSelfContainedComponents :: UNWIND , "linker" => LinkSelfContainedComponents :: LINKER , "sanitizers" => LinkSelfContainedComponents :: SANITIZERS , "mingw" => LinkSelfContainedComponents :: MINGW , _ => { return Err (format ! ("'{s}' is not a valid link-self-contained component, expected 'crto', 'libc', 'unwind', 'linker', 'sanitizers', 'mingw'")) ; } }) } }
    };
}

impl_472!();