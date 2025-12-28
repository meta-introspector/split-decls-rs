macro_rules! deps {
    () => {
        NamedTempFile!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg (windows)] impl < F : AsRawHandle > AsRawHandle for NamedTempFile < F > { # [inline] fn as_raw_handle (& self) -> RawHandle { self . as_file () . as_raw_handle () } }
    };
}

impl_61!();