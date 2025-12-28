macro_rules! file {
    () => {
        # [cfg (feature = "full")] mod file ;
    };
}

file!()