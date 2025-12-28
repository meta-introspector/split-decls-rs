macro_rules! deps {
    () => {
        TargetOptions!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl TargetOptions { pub fn supports_comdat (& self) -> bool { ! self . is_like_aix && ! self . is_like_darwin } }
    };
}

impl_539!();