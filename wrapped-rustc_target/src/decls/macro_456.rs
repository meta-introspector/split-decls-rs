macro_rules! macro_456 {
    () => {
        crate :: target_spec_enum ! { pub enum LldFlavor { Wasm = "wasm" , Ld64 = "darwin" , Ld = "gnu" , Link = "link" , } parse_error_type = "LLD flavor" ; }
    };
}

macro_456!();