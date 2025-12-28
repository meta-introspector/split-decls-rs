macro_rules! sendfile {
    () => {
        # [cfg (target_os = "linux")] mod sendfile ;
    };
}

sendfile!()