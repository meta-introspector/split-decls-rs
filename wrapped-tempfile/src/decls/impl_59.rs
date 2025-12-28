macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (any (unix , target_os = "wasi"))] impl < F : AsRawFd > AsRawFd for NamedTempFile < F > { # [inline] fn as_raw_fd (& self) -> RawFd { self . as_file () . as_raw_fd () } }
    };
}

impl_59!()