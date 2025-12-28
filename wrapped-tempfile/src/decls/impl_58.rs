macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (any (unix , target_os = "wasi"))] impl < F : AsFd > AsFd for NamedTempFile < F > { fn as_fd (& self) -> BorrowedFd < '_ > { self . as_file () . as_fd () } }
    };
}

impl_58!();