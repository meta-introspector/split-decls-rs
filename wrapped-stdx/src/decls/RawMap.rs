macro_rules! deps {
    () => {
        TypeIdHasher!();
    };
}

macro_rules! RawMap {
    () => {
        deps!();
        # [doc = " Raw access to the underlying `HashMap`."] # [doc = ""] # [doc = " This alias is provided for convenience because of the ugly third generic parameter."] # [expect (clippy :: disallowed_types , reason = "Uses a custom hasher")] pub type RawMap < A > = hash_map :: HashMap < TypeId , Box < A > , BuildHasherDefault < TypeIdHasher > > ;
    };
}

RawMap!()