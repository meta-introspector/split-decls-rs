macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < F > AsRef < Path > for NamedTempFile < F > { # [inline] fn as_ref (& self) -> & Path { self . path () } }
    };
}

impl_42!();