macro_rules! RustcVersion {
    () => {
        struct RustcVersion { major : u16 , minor : u16 , patch : u16 , }
    };
}

RustcVersion!()