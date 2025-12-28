macro_rules! blocking_check {
    () => {
        # [cfg (feature = "net")] mod blocking_check ;
    };
}

blocking_check!();