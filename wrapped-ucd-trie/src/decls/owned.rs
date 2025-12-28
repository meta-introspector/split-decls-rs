macro_rules! owned {
    () => {
        # [cfg (feature = "std")] mod owned ;
    };
}

owned!();