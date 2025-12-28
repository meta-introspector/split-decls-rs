macro_rules! DirRootInner {
    () => {
        # [derive (Debug)] enum DirRootInner { None , Immutable (std :: path :: PathBuf) , # [cfg (feature = "dir")] MutablePath (std :: path :: PathBuf) , # [cfg (feature = "dir")] MutableTemp { temp : tempfile :: TempDir , path : std :: path :: PathBuf , } , }
    };
}

DirRootInner!();