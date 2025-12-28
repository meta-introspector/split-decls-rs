macro_rules! copy_file_range {
    () => {
        # [cfg (linux_kernel)] mod copy_file_range ;
    };
}

copy_file_range!()