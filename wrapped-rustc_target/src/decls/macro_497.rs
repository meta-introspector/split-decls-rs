macro_rules! macro_497 {
    () => {
        crate :: target_spec_enum ! { # [doc = " Everything is flattened to a single enum to make the json encoding/decoding less annoying."] pub enum LinkOutputKind { # [doc = " Dynamically linked non position-independent executable."] DynamicNoPicExe = "dynamic-nopic-exe" , # [doc = " Dynamically linked position-independent executable."] DynamicPicExe = "dynamic-pic-exe" , # [doc = " Statically linked non position-independent executable."] StaticNoPicExe = "static-nopic-exe" , # [doc = " Statically linked position-independent executable."] StaticPicExe = "static-pic-exe" , # [doc = " Regular dynamic library (\"dynamically linked\")."] DynamicDylib = "dynamic-dylib" , # [doc = " Dynamic library with bundled libc (\"statically linked\")."] StaticDylib = "static-dylib" , # [doc = " WASI module with a lifetime past the _initialize entry point"] WasiReactorExe = "wasi-reactor-exe" , } parse_error_type = "CRT object kind" ; }
    };
}

macro_497!()