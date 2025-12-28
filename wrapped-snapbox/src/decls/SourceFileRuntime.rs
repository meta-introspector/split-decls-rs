macro_rules! deps {
    () => {
        Patchwork!();
    };
}

macro_rules! SourceFileRuntime {
    () => {
        deps!();
        struct SourceFileRuntime { path : std :: path :: PathBuf , original_text : String , patchwork : Patchwork , }
    };
}

SourceFileRuntime!();