macro_rules! IdentPrintMode {
    () => {
        pub enum IdentPrintMode { Normal , RawIdent , RawLifetime , }
    };
}

IdentPrintMode!()