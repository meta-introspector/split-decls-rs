macro_rules! deps {
    () => {
        TempPath!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl AsRef < Path > for TempPath { fn as_ref (& self) -> & Path { & self . path } }
    };
}

impl_38!()