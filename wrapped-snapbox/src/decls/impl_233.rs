macro_rules! deps {
    () => {
        RedactedValue!();
        RedactedValueInner!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl From < PathBuf > for RedactedValue { fn from (inner : PathBuf) -> Self { if inner . as_os_str () . is_empty () { Self { inner : None } } else { let native = match inner . into_os_string () . into_string () { Ok (s) => s , Err (os) => PathBuf :: from (os) . display () . to_string () , } ; let normalized = crate :: filter :: normalize_paths (& native) ; Self { inner : Some (RedactedValueInner :: Path { native , normalized }) , } } } }
    };
}

impl_233!()