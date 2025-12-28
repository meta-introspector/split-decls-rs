macro_rules! NULL_DEVICE {
    () => {
        # [cfg (not (windows))] const NULL_DEVICE : & str = "/dev/null" ;
    };
}

NULL_DEVICE!();