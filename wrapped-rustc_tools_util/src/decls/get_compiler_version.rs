macro_rules! get_compiler_version {
    () => {
        # [must_use] pub fn get_compiler_version () -> Option < String > { let compiler = std :: option_env ! ("RUSTC") . unwrap_or ("rustc") ; get_output (compiler , & ["-V"]) }
    };
}

get_compiler_version!();