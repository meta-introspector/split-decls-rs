macro_rules! RustcMirAttrs {
    () => {
        # [derive (Default)] struct RustcMirAttrs { basename_and_suffix : Option < PathBuf > , formatter : Option < Symbol > , }
    };
}

RustcMirAttrs!();