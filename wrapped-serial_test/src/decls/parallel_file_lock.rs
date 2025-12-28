macro_rules! parallel_file_lock {
    () => {
        # [cfg (feature = "file_locks")] mod parallel_file_lock ;
    };
}

parallel_file_lock!()