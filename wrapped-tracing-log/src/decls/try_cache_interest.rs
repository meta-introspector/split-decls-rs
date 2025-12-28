macro_rules! try_cache_interest {
    () => {
        # [cfg (not (all (feature = "interest-cache" , feature = "std")))] fn try_cache_interest (_ : & log :: Metadata < '_ > , callback : impl FnOnce () -> bool) -> bool { callback () }
    };
}

try_cache_interest!()