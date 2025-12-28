macro_rules! macro_519 {
    () => {
        crate :: target_spec_enum ! { pub enum BinaryFormat { Coff = "coff" , Elf = "elf" , MachO = "mach-o" , Wasm = "wasm" , Xcoff = "xcoff" , } parse_error_type = "binary format" ; }
    };
}

macro_519!()