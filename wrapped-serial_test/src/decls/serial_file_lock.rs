macro_rules! serial_file_lock {
    () => {
        # [cfg (feature = "file_locks")] mod serial_file_lock ;
    };
}

serial_file_lock!()