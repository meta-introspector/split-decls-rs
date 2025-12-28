macro_rules! RustcTarget {
    () => {
        # [derive (Deserialize)] struct RustcTarget { src_path : PathBuf , }
    };
}

RustcTarget!();