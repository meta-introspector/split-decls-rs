macro_rules! deps {
    () => {
        LinkSelfContainedDefault!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        # [doc = " Parses a backwards-compatible `-Clink-self-contained` option string, without components."] impl FromStr for LinkSelfContainedDefault { type Err = String ; fn from_str (s : & str) -> Result < LinkSelfContainedDefault , Self :: Err > { Ok (match s { "false" => LinkSelfContainedDefault :: False , "true" | "wasm" => LinkSelfContainedDefault :: True , "musl" => LinkSelfContainedDefault :: InferredForMusl , "mingw" => LinkSelfContainedDefault :: InferredForMingw , _ => { return Err (format ! ("'{s}' is not a valid `-Clink-self-contained` default. \
                        Use 'false', 'true', 'wasm', 'musl' or 'mingw'" ,)) ; } }) } }
    };
}

impl_464!();