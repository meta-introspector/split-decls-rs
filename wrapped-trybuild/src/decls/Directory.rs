macro_rules! Directory {
    () => {
        # [derive (Clone , Debug , Serialize)] # [serde (transparent)] pub (crate) struct Directory { path : PathBuf , }
    };
}

Directory!()