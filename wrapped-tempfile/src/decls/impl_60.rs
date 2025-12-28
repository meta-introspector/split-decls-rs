macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (windows)] impl < F : AsHandle > AsHandle for NamedTempFile < F > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { self . as_file () . as_handle () } }
    };
}

impl_60!()