macro_rules! Walk {
    () => {
        # [doc = " Recursively walk a path"] # [doc = ""] # [doc = " Note: Ignores `.keep` files"] # [cfg (feature = "dir")] pub struct Walk { inner : walkdir :: IntoIter , }
    };
}

Walk!()